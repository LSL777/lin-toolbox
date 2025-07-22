<script setup lang="ts">
import Database from '@tauri-apps/plugin-sql';
import {onMounted, onUnmounted, reactive, ref, watch} from 'vue';
import {getLabelByValue} from "@/utils/KvUtil.ts";
import {invoke} from "@tauri-apps/api/core";
import {dayjs, ElMessage, ElMessageBox, FormInstance, FormRules} from "element-plus";

interface TodoTask {
  id: string;
  content: string;
  remind_time: string;
  cron_expr: String;
}

interface TodoItem {
  id: string;
  content: string;
  remind_time: string;
  // 周期性任务 1：周期性任务 0：一次性任务
  is_recurring: string;
  cron_expression: string;
  status: number;
  effective: number;
  create_time: string;
  update_time: string;
}

const recurringOptions = [
  {
    value: 1,
    label: '是'
  },
  {
    value: 0,
    label: '否'
  }
];

const statusOptions = [
  {
    value: 0,
    label: '待完成'
  },
  {
    value: 1,
    label: '进行中'
  },
  {
    value: 2,
    label: '已完成'
  }
];

const effectiveOptions = [
  {
    value: 1,
    label: '有效'
  },
  {
    value: 0,
    label: '无效'
  }
];

const queryFormRef = ref<FormInstance>()

const db = ref<Database | null>(null);

const form = reactive({
  content: '',
  startTime: '',
  endTime: '',
  isRecurring: '',
  status: '',
  effective: ''
});

const addFormRef = ref<FormInstance>()
const addForm = reactive<TodoItem>({
  id: '',
  content: '',
  remind_time: '',
  is_recurring: '',
  cron_expression: '',
  status: 0,
  effective: 1,
  create_time: '',
  update_time: ''
});

// @ts-ignore
const checkRemindTime = (rule: any, value: any, callback: any) => {
  // 如果是周期性任务直接放开
  if (addForm.is_recurring) {
    callback()
  } else {
    if (!value) {
      callback(new Error('非周期性任务必须填写提醒时间'))
    } else {
      if (dayjs(value).isBefore(dayjs().format('YYYY-MM-DD HH:mm:ss'))) {
        callback(new Error('提醒时间不能早于当前时间'))
      } else {
        callback()
      }
    }
  }
}

// @ts-ignore
const checkCronExpression = (rule: any, value: any, callback: any) => {
  // 非周期任务直接放开
  if (!addForm.is_recurring) {
    callback()
  } else {
    if (!value) {
      callback(new Error('周期性任务cron表达式不能为空'))
    } else {
      callback()
    }
  }
}

const addFormRules = reactive<FormRules<typeof addForm>>({
  content: [
    {required: true, message: '请输入待办内容', max: 1023, trigger: ['blur', 'change']}
  ],
  remind_time: [
    {validator: checkRemindTime, trigger: ['blur', 'change']}
  ],
  cron_expression: [
    {validator: checkCronExpression, trigger: ['blur', 'change']}
  ]
})

const tableData = ref<TodoItem[]>([])

const addDialogFormVisible = ref(false)
const showRemindTime = ref(false)
const showCronExpression = ref(false)

const getTableData = async () => {
  let selectSql = "select * from todo_list where 1 = 1 "
  let condition = ""
  if (form.content !== undefined && form.content !== null && form.content !== '') {
    condition += `and content like '%${form.content}%' `
  }
  if (form.startTime !== undefined && form.startTime !== null && form.startTime !== '') {
    condition += `and remind_time >= '${form.startTime}'`
  }
  if (form.endTime !== undefined && form.endTime !== null && form.endTime !== '') {
    condition += `and remind_time <= '${form.endTime}'`
  }
  if (form.isRecurring !== undefined && form.isRecurring !== null && form.isRecurring !== '') {
    condition += `and is_recurring = ${form.isRecurring}`
  }
  if (form.status !== undefined && form.status !== null && form.status !== '') {
    condition += `and status = ${form.status}`
  }
  if (form.effective !== undefined && form.effective !== null && form.effective !== '') {
    condition += `and effective = ${form.effective}`
  }
  if (condition !== "") {
    selectSql += condition + " order by create_time desc"
  } else {
    selectSql += " order by create_time desc"
  }
  tableData.value = (await db.value?.select(selectSql)) as TodoItem[] || [];
}

const openAddDialog = () => {
  addDialogFormVisible.value = true
  addFormRef.value?.resetFields()
}

const submitQueryForm = async (formEl: FormInstance | undefined) => {
  if (!formEl) return
  await getTableData()
}

const resetForm = (formEl: FormInstance | undefined) => {
  if (!formEl) return
  formEl.resetFields()
  getTableData()
}

const handleDelete = (id: number | string) => {
  ElMessageBox.confirm(
      '确认删除该数据?',
      '警告',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning',
      }
  )
      .then(() => {
        doDelete(id)
      })
      .catch(() => {
        ElMessage({
          type: 'info',
          message: '取消删除',
        })
      })
}

const submitAddForm = (formEl: FormInstance | undefined) => {
  if (!formEl) return
  formEl.validate((valid) => {
    if (valid) {
      doSubmitForm()
    } else {
      console.log('error submit!')
    }
  })
}

const doDelete = async (id: number | string) => {
  const result = await db.value?.execute("delete from todo_list where id = $1", [id])
  if (result !== undefined && result !== null && result.rowsAffected === 1) {
    ElMessage({message: '删除成功', type: 'success'})
    resetForm(queryFormRef.value)
  }
}

const doSubmitForm = async () => {
  addForm.id = await invoke("generate_snowflake_id")
  const currDateTime = dayjs().format('YYYY-MM-DD HH:mm:ss');
  addForm.create_time = currDateTime
  addForm.update_time = currDateTime
  const task: TodoTask = {
    id: addForm.id,
    content: addForm.content,
    remind_time: addForm.remind_time,
    cron_expr: addForm.cron_expression
  };

  let result = null
  // @ts-ignore
  if (addForm.is_recurring === 1) {
    result = await db.value?.execute(
        "INSERT into todo_list (id, content, remind_time, is_recurring, cron_expression, status, effective, create_time, update_time)" +
        " VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
        [addForm.id, addForm.content, null, addForm.is_recurring, addForm.cron_expression, addForm.status, addForm.effective, addForm.create_time, addForm.update_time],
    );
    await createCronTaskToRust(task)
  } else {
    result = await db.value?.execute(
        "INSERT into todo_list (id, content, remind_time, is_recurring, cron_expression, status, effective, create_time, update_time)" +
        " VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
        [addForm.id, addForm.content, addForm.remind_time, addForm.is_recurring, null, addForm.status, addForm.effective, addForm.create_time, addForm.update_time],
    );
    await createAperiodicityTaskToRust(task)
  }
  if (result != null && result.rowsAffected === 1) {
    closeAddDialog()
    ElMessage({message: '创建待办事项成功', type: 'success'})
    resetForm(queryFormRef.value)
  } else {
    ElMessage({message: '创建待办事项失败', type: 'error'})
  }
}

const createAperiodicityTaskToRust = async (todoTask: TodoTask) => {
  await invoke('schedule_reminder', {todo: todoTask});
}

const createCronTaskToRust = async (todoTask: TodoTask) => {
  await invoke('schedule_cron_task', {task: todoTask});
}

const closeAddDialog = () => {
  showRemindTime.value = false
  showCronExpression.value = false
  addDialogFormVisible.value = false
}

onMounted(async () => {
  try {
    db.value = await Database.load('sqlite:test.db');
    await getTableData()
  } catch (error) {
    console.error('数据库加载失败:', error);
  }
});

watch(
    () => addForm.is_recurring,
    (newValue) => {
      // @ts-ignore
      if (newValue === 1) {
        showCronExpression.value = true
        showRemindTime.value = false
        addForm.remind_time = ''
        addForm.cron_expression = ''
        // @ts-ignore
      } else if (newValue === 0) {
        showCronExpression.value = false
        showRemindTime.value = true
        addForm.remind_time = ''
        addForm.cron_expression = ''
      } else {
        // 既不是 1 也不是 0，全部隐藏
        showCronExpression.value = false
        showRemindTime.value = false
      }
    }
)

// CREATE TABLE todo_list (
// id bigint primary key, -- 待办事项ID
// content varchar(1024), -- 待办事项内容
// remind_time datetime, -- 提醒时间
// is_recurring boolean, -- 是否为周期性任务
// cron_expression varchar(64), -- cron表达式，周期性任务时有效,
// status tinyint, -- 状态：0：待完成 1：进行中 2：已完成
// effective boolean, -- 0: 无效 1：有效
// create_time datetime, -- 创建时间
// update_time datetime -- 修改时间
// )

onUnmounted(() => {
  // 关闭数据库连接（如果需要）
  if (db.value) {
    db.value.close().catch(err => console.error('关闭数据库失败:', err));
  }
});
</script>

<template>
  <div class="general-box">
    <h4>待办事项</h4>
    <el-form :model="form" label-width="auto" style="width: 100%; max-width: 1280px;" :inline="true" ref="queryFormRef">
      <el-form-item label="待办内容" prop="content">
        <el-input v-model="form.content"/>
      </el-form-item>
      <el-form-item label="起始时间" prop="startTime">
        <el-date-picker
            v-model="form.startTime"
            type="datetime"
            placeholder="请选择起始时间"
            value-format="YYYY-MM-DD HH:mm:ss"
        />
      </el-form-item>
      <el-form-item label="结束时间" prop="endTime">
        <el-date-picker
            v-model="form.endTime"
            type="datetime"
            placeholder="请选择终止时间"
            value-format="YYYY-MM-DD HH:mm:ss"
        />
      </el-form-item>
      <el-form-item label="周期性任务" prop="isRecurring">
        <el-select v-model="form.isRecurring" placeholder="请选择是否为周期性任务" style="width: 240px">
          <el-option
              v-for="item in recurringOptions"
              :key="item.value"
              :label="item.label"
              :value="item.value"
          />
        </el-select>
      </el-form-item>
      <el-form-item label="状态" prop="status">
        <el-select v-model="form.status" placeholder="请选择状态" style="width: 240px">
          <el-option
              v-for="item in statusOptions"
              :key="item.value"
              :label="item.label"
              :value="item.value"
          />
        </el-select>
      </el-form-item>
      <el-form-item label="有效性" prop="effective">
        <el-select v-model="form.effective" placeholder="请选择有效性" style="width: 240px">
          <el-option
              v-for="item in effectiveOptions"
              :key="item.value"
              :label="item.label"
              :value="item.value"
          />
        </el-select>
      </el-form-item>
      <el-form-item>
        <el-button type="primary" @click="submitQueryForm(queryFormRef)">
          查询
        </el-button>
        <el-button @click="resetForm(queryFormRef)">重置</el-button>
        <el-button @click="openAddDialog">新增</el-button>
      </el-form-item>
    </el-form>

    <el-table :data="tableData" border style="width: 100%">
      <el-table-column prop="content" label="内容"/>
      <el-table-column prop="remind_time" label="提醒时间"/>
      <el-table-column prop="is_recurring" label="周期性任务" width="120px">
        <template #default="scope">
          {{ getLabelByValue(scope.row.is_recurring, recurringOptions) }}
        </template>
      </el-table-column>
      <el-table-column prop="cron_expression" label="cron表达式"/>
      <el-table-column prop="status" label="状态" width="96px">
        <template #default="scope">
          {{ getLabelByValue(scope.row.status, statusOptions) }}
        </template>
      </el-table-column>
      <el-table-column prop="effective" label="有效性" width="72px">
        <template #default="scope">
          {{ getLabelByValue(scope.row.effective, effectiveOptions) }}
        </template>
      </el-table-column>
      <el-table-column prop="create_time" label="创建时间"/>
      <el-table-column prop="update_time" label="更新时间"/>
      <el-table-column fixed="right" label="操作" width="72px">
        <template #default="scope">
          <el-button link type="primary" @click="handleDelete(scope.row.id)">
            删除
          </el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>

  <!-- 添加对话框 start  -->
  <el-dialog v-model="addDialogFormVisible" title="添加待办事项" width="560" @close="closeAddDialog">
    <el-form :model="addForm" label-width="auto" style="width: 100%; max-width: 1280px;" ref="addFormRef"
             :rules="addFormRules">
      <el-form-item label="待办事项内容" prop="content" required>
        <el-input v-model="addForm.content"/>
      </el-form-item>
      <el-form-item label="周期性任务" prop="is_recurring">
        <el-select v-model="addForm.is_recurring" placeholder="请选择是否为周期性任务" style="width: 240px">
          <el-option
              v-for="item in recurringOptions"
              :key="item.value"
              :label="item.label"
              :value="item.value"
          />
        </el-select>
      </el-form-item>
      <el-form-item label="cron表达式" prop="cron_expression" v-show="showCronExpression">
        <el-input v-model="addForm.cron_expression"/>
      </el-form-item>
      <el-form-item label="提醒时间" prop="remind_time" v-show="showRemindTime">
        <el-date-picker
            v-model="addForm.remind_time"
            type="datetime"
            placeholder="请选择提醒时间"
            format="YYYY-MM-DD HH:mm:ss"
            value-format="YYYY-MM-DD HH:mm:ss"
        />
      </el-form-item>
      <el-form-item label="状态" prop="status">
        <el-select v-model="addForm.status" placeholder="请选择状态" style="width: 240px" disabled>
          <el-option
              v-for="item in statusOptions"
              :key="item.value"
              :label="item.label"
              :value="item.value"
          />
        </el-select>
      </el-form-item>
      <el-form-item label="有效性" prop="effective">
        <el-select v-model="addForm.effective" placeholder="请选择有效性" style="width: 240px" disabled>
          <el-option
              v-for="item in effectiveOptions"
              :key="item.value"
              :label="item.label"
              :value="item.value"
          />
        </el-select>
      </el-form-item>
      <el-form-item>
        <el-button type="primary" @click.stop="submitAddForm(addFormRef)">保存</el-button>
        <el-button @click.stop="closeAddDialog">取消</el-button>
      </el-form-item>
    </el-form>
  </el-dialog>
  <!-- 添加对话框 end  -->
</template>

<style scoped>

</style>