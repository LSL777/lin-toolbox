<script setup lang="ts">
import {reactive, ref} from 'vue'
import {invoke} from "@tauri-apps/api/core";
import {FormRules, FormInstance} from "element-plus";

interface NetResult {
  result: Boolean;
  reason: string;
}

const portOpenResultMsg = ref('')
const portRuleFormRef = ref<FormInstance>()
const portForm = reactive({
  host: '',
  port: '',
})

const checkPort = (rule: any, value: any, callback: any) => {
  const regex = /^(6553[0-5]|655[0-2][0-9]|65[0-4][0-9]{2}|6[0-4][0-9]{3}|[1-5][0-9]{4}|[1-9][0-9]{0,3}|0)$/
  if (value.match(regex)) {
    callback()
  } else {
    callback(new Error('端口范围应该在0-65535'))
  }
}

const portFormRules = reactive<FormRules<typeof portForm>>({
  host: [
    {required: true, message: '请输入IP地址', trigger: ['blur', 'change']}
  ],
  port: [
    {required: true, message: '请输入端口', trigger: ['blur', 'change']},
    {validator: checkPort, trigger: ['blur', 'change']}
  ],
})

const handleCheckPortOpen = (formEl: FormInstance | undefined) => {
  if (!formEl) return
  formEl.validate((valid) => {
    if (valid) {
      submitPortForm()
    } else {
      console.log('error submit!')
    }
  })
}

const submitPortForm = async () => {
  portOpenResultMsg.value = ''
  try {
    if (portForm.port === null || portForm.port.trim === undefined) return false; // 防止 null 或非字符串
    const port = Number(portForm.port.trim());
    const res: NetResult = await invoke('is_port_open', {host: portForm.host, port: port})
    if (res.result) {
      portOpenResultMsg.value = portForm.host + ':' + portForm.port + ' 开放'
    } else {
      portOpenResultMsg.value = portForm.host + ':' + portForm.port + ' 关闭'
    }
  } catch (error) {
    portOpenResultMsg.value = portForm.host + ':' + portForm.port + ' 关闭'
  }
}
</script>

<template>
  <div class="general-box">
    <h4>端口开放测试</h4>
    <el-form ref="portRuleFormRef" :model="portForm" :rules="portFormRules" label-width="auto" style="width: 100%"
             :inline="true">
      <el-form-item label="IP地址" prop="host" required>
        <el-input v-model="portForm.host"/>
      </el-form-item>
      <el-form-item label="端口" prop="port" required>
        <el-input v-model="portForm.port"/>
      </el-form-item>
      <el-form-item>
        <el-button type="primary" @click="handleCheckPortOpen(portRuleFormRef)">测试</el-button>
      </el-form-item>
    </el-form>
    <div style="height: 64px">
      {{ portOpenResultMsg }}
    </div>
  </div>
</template>

<style scoped>

</style>