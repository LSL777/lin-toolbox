<script setup lang="ts">
import {reactive, ref} from 'vue'
import {invoke} from "@tauri-apps/api/core";
import {CopyDocument} from "@element-plus/icons-vue";
import IconFont from "@/components/IconFont.vue";

const form = reactive({
  phone: '',
  idCard: '',
  name: '',
  bankNumber: '',
  bankName: ''
})

const count = ref()

const tableData = ref([])

async function getPhone() {
  form.phone = await invoke("build_phone")
}

async function getIdCard() {
  form.idCard = await invoke("build_id_card")
}

async function getName() {
  form.name = await invoke("build_name")
}

async function getBankInfo() {
  const bankInfo: Array<string> = await invoke("build_bank_info")
  form.bankNumber = bankInfo[0]
  form.bankName = bankInfo[1]
}

const getTableData = async () => {
  tableData.value = await invoke("build_table_data", {count: count.value})
}
</script>

<template>
  <h2>随机生成数据</h2>
  <el-form :model="form" label-width="auto">
    <el-form-item label="名字">
      <el-input v-model="form.name" id="nameInput" size="large" style="width: 60%" readonly/>
      <el-button @click.stop="getName" size="large" style="margin-left: 8px">重新生成</el-button>
      <el-button size="large" v-copy="{ selector: '#nameInput' }" type="primary" :icon="CopyDocument">复制内容
      </el-button>
    </el-form-item>
    <el-form-item label="手机号">
      <el-input v-model="form.phone" id="phoneInput" size="large" style="width: 60%" readonly/>
      <el-button @click.stop="getPhone" size="large" style="margin-left: 8px">重新生成</el-button>
      <el-button size="large" v-copy="{ selector: '#phoneInput' }" type="primary" :icon="CopyDocument">复制内容
      </el-button>
    </el-form-item>
    <el-form-item label="身份证号码">
      <el-input v-model="form.idCard" id="idCardInput" size="large" style="width: 60%" readonly/>
      <el-button @click.stop="getIdCard" size="large" style="margin-left: 8px">重新生成</el-button>
      <el-button size="large" v-copy="{ selector: '#idCardInput' }" type="primary" :icon="CopyDocument">复制内容
      </el-button>
    </el-form-item>
    <el-form-item label="银行卡信息">
      <el-input v-model="form.bankNumber" id="bankNumberInput" size="large" style="width: 40%; margin-right: 8px;"
                readonly/>
      <el-input v-model="form.bankName" size="large" style="width: 25%" readonly/>
      <el-button @click.stop="getBankInfo" size="large" style="margin-left: 8px">重新生成</el-button>
      <el-button size="large" v-copy="{ selector: '#bankNumberInput' }" type="primary" :icon="CopyDocument">复制卡号
      </el-button>
    </el-form-item>
  </el-form>

  <el-divider/>

  <h2>随机生成批量数据</h2>
  <div class="tbl-form">
    <el-input-number v-model="count" :step="1" :max="20" :precision="0" step-strictly style="margin-right: 8px"
                     size="default"/>
    <el-button type="primary" @click.stop="getTableData" size="default">生成</el-button>
  </div>
  <el-table :data="tableData" border style="width: 100%">
    <el-table-column prop="name" label="姓名">
      <template #default="scope">
        <span>{{ scope.row.name }}</span>
        <IconFont name="fuzhi" v-copy="scope.row.name"/>
      </template>
    </el-table-column>
    <el-table-column prop="phone" label="手机号" width="180">
      <template #default="scope">
        <span>{{ scope.row.phone }}</span>
        <IconFont name="fuzhi" v-copy="scope.row.phone"/>
      </template>
    </el-table-column>
    <el-table-column prop="id_card" label="身份证号码">
      <template #default="scope">
        <span>{{ scope.row.id_card }}</span>
        <IconFont name="fuzhi" v-copy="scope.row.id_card"/>
      </template>
    </el-table-column>
    <el-table-column prop="bank_number" label="银行卡号">
      <template #default="scope">
        <span>{{ scope.row.bank_number }}</span>
        <IconFont name="fuzhi" v-copy="scope.row.bank_number"/>
      </template>
    </el-table-column>
    <el-table-column prop="bank_name" label="银行卡所属"/>
  </el-table>
</template>

<style scoped>
.tbl-form {
  display: flex;
  align-items: center;
  margin-bottom: 12px;
}

:deep(.el-form-item__label-wrap) {
  align-items: center;
  align-content: center;
}

:deep(.el-form-item__label) {
  font-size: 16px;
}

</style>