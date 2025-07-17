<script setup lang="ts">
import IconFont from "@/components/IconFont.vue";
import {ref} from "vue";
import {ArrowDown} from "@element-plus/icons-vue";
import {ElMessage} from "element-plus";

// @ts-ignore
const tableRowClassName = ({row, rowIndex}) => {
  if (rowIndex % 2 === 1) {
    return 'warning-row'
  } else if (rowIndex % 2 === 0) {
    return 'success-row'
  }
  return ''
}
const tableData = [
  {
    "usage": "数字",
    "regex": "^\\d+$",
    "example": "12345",
    "description": "只允许非负整数（0或正整数）"
  },
  {
    "usage": "n位数字",
    "regex": "^\\d{n}$",
    "example": "1234 (n=4)",
    "description": "恰好 n 位数字"
  },
  {
    "usage": "至少n位数字",
    "regex": "^\\d{n,}$",
    "example": "12345 (n=3)",
    "description": "至少 n 位数字"
  },
  {
    "usage": "m-n位数字",
    "regex": "^\\d{m,n}$",
    "example": "1234 (m=2,n=4)",
    "description": "m 到 n 位数字"
  },
  {
    "usage": "零和非零开头数字",
    "regex": "^(0|[1-9]\\d*)$",
    "example": "0, 123",
    "description": "0 或非零开头的整数"
  },
  {
    "usage": "非零开头最多两位小数",
    "regex": "^[1-9]\\d*(\\.\\d{1,2})?$",
    "example": "12.34",
    "description": "正整数或带1-2位小数"
  },
  {
    "usage": "正负数带1-2位小数",
    "regex": "^-?\\d+(\\.\\d{1,2})?$",
    "example": "-12.34, 56.7",
    "description": "正负数，1-2位小数"
  },
  {
    "usage": "正数、负数、小数",
    "regex": "^[-+]?\\d+(\\.\\d+)?$",
    "example": "+12.3, -0.5",
    "description": "正负整数或小数"
  },
  {
    "usage": "两位小数正实数",
    "regex": "^\\d+(\\.\\d{2})?$",
    "example": "12, 12.34",
    "description": "正整数或两位小数"
  },
  {
    "usage": "1~3位小数正实数",
    "regex": "^\\d+(\\.\\d{1,3})?$",
    "example": "12.3, 12.345",
    "description": "正整数或1-3位小数"
  },
  {
    "usage": "非零正整数",
    "regex": "^[1-9]\\d*$",
    "example": "123",
    "description": "不含0的正整数"
  },
  {
    "usage": "非零负整数",
    "regex": "^-[1-9]\\d*$",
    "example": "-123",
    "description": "不含0的负整数"
  },
  {
    "usage": "非负整数",
    "regex": "^\\d+$",
    "example": "0, 123",
    "description": "0或正整数"
  },
  {
    "usage": "非正整数",
    "regex": "^(-\\d+|0)$",
    "example": "-123, 0",
    "description": "0或负整数"
  },
  {
    "usage": "非负浮点数",
    "regex": "^\\d+(\\.\\d+)?$",
    "example": "0, 12.3",
    "description": "0或正浮点数"
  },
  {
    "usage": "非正浮点数",
    "regex": "^(-\\d+(\\.\\d+)?|0+(\\.0+)?)$",
    "example": "-12.3, 0, 0.0",
    "description": "0或负浮点数"
  },
  {
    "usage": "正浮点数",
    "regex": "^(([1-9]\\d*)|0)\\.\\d+$",
    "example": "0.1, 12.3",
    "description": "大于0的浮点数"
  },
  {
    "usage": "负浮点数",
    "regex": "^-(([1-9]\\d*)|0)\\.\\d+$",
    "example": "-0.1, -12.3",
    "description": "小于0的浮点数"
  },
  {
    "usage": "浮点数",
    "regex": "^-?\\d+(\\.\\d+)?$",
    "example": "-12.3, 0, 45.6",
    "description": "正负浮点数"
  },
  {
    "usage": "汉字",
    "regex": "^[\\u4e00-\\u9fa5]+$",
    "example": "你好世界",
    "description": "只允许汉字，长度至少1"
  },
  {
    "usage": "英文和数字",
    "regex": "^[A-Za-z0-9]+$",
    "example": "abc123",
    "description": "只允许英文和数字"
  },
  {
    "usage": "4-40位英文和数字",
    "regex": "^[A-Za-z0-9]{4,40}$",
    "example": "abc12345",
    "description": "4-40位英文和数字"
  },
  {
    "usage": "长度为3-20的所有字符",
    "regex": "^.{3,20}$",
    "example": "abc, 12345678901234567890",
    "description": "任意字符，3-20位"
  },
  {
    "usage": "26个英文字母",
    "regex": "^[A-Za-z]+$",
    "example": "abcXYZ",
    "description": "只允许大小写英文字母"
  },
  {
    "usage": "26个大写英文字母",
    "regex": "^[A-Z]+$",
    "example": "ABCXYZ",
    "description": "只允许大写英文字母"
  },
  {
    "usage": "26个小写英文字母",
    "regex": "^[a-z]+$",
    "example": "abcxyz",
    "description": "只允许小写英文字母"
  },
  {
    "usage": "数字和26个英文字母",
    "regex": "^[A-Za-z0-9]+$",
    "example": "abc123",
    "description": "只允许数字和英文字母"
  },
  {
    "usage": "数字、字母或下划线",
    "regex": "^\\w+$",
    "example": "abc_123",
    "description": "数字、字母或下划线"
  },
  {
    "usage": "3-20位数字、字母或下划线",
    "regex": "^\\w{3,20}$",
    "example": "abc_123",
    "description": "3-20位数字、字母或下划线"
  },
  {
    "usage": "中文、英文、数字、下划线",
    "regex": "^[\\u4E00-\\u9FA5A-Za-z0-9_]+$",
    "example": "你好abc_123",
    "description": "允许中英文、数字、下划线"
  },
  {
    "usage": "中文、英文、数字（不含下划线）",
    "regex": "^[\\u4E00-\\u9FA5A-Za-z0-9]+$",
    "example": "你好abc123",
    "description": "允许中英文、数字，不含下划线"
  },
  {
    "usage": "可输入除%&',;=?$\"等字符外的内容",
    "regex": "^[^%&',;=?$\\x22]+$",
    "example": "abc123!@#",
    "description": "不含%&',;=?$\"等字符"
  },
  {
    "usage": "禁止输入含有~的字符",
    "regex": "^[^~]+$",
    "example": "abc123",
    "description": "不含~字符"
  },
  {
    "usage": "Email地址",
    "regex": "^\\w+([-+.']\\w+)*@\\w+([-.]\\w+)*\\.\\w+([-.]\\w+)*$",
    "example": "test@mail.com",
    "description": "常用邮箱格式"
  },
  {
    "usage": "域名",
    "regex": "[a-zA-Z0-9][-a-zA-Z0-9]{0,62}(\\.[a-zA-Z0-9][-a-zA-Z0-9]{0,62})+\\.?",
    "example": "www.example.com",
    "description": "支持多级域名"
  },
  {
    "usage": "Internet URL",
    "regex": "^[a-zA-Z]+://[^\\s]+$",
    "example": "https://abc.com",
    "description": "常用URL格式"
  },
  {
    "usage": "手机号码",
    "regex": "^(13[0-9]|14[01456879]|15[0-35-9]|16[2567]|17[0-8]|18[0-9]|19[0-35-9])\\d{8}$",
    "example": "13812345678",
    "description": "中国大陆手机号"
  },
  {
    "usage": "电话号码（区号-号码）",
    "regex": "^(\\d{3,4}-)?\\d{7,8}$",
    "example": "010-12345678",
    "description": "区号可选"
  },
  {
    "usage": "国内电话号码",
    "regex": "\\d{3}-\\d{8}|\\d{4}-\\d{7}",
    "example": "021-12345678",
    "description": "常见座机格式"
  },
  {
    "usage": "支持分机号的电话",
    "regex": "((\\d{11})|((\\d{7,8})|(\\d{3,4}-\\d{7,8})|(\\d{3,4}-\\d{7,8}-\\d{1,4})|(\\d{7,8}-\\d{1,4})))$",
    "example": "010-12345678-123",
    "description": "支持分机号"
  },
  {
    "usage": "身份证号",
    "regex": "(^[1-9]\\d{5}(18|19|([23]\\d))\\d{2}((0[1-9])|(10|11|12))(([0-2][1-9])|10|20|30|31)\\d{3}[0-9Xx]$)|(^[1-9]\\d{5}\\d{2}((0[1-9])|(10|11|12))(([0-2][1-9])|10|20|30|31)\\d{2}$)",
    "example": "110101199003071234",
    "description": "15/18位，末位可为X"
  },
  {
    "usage": "合法账号",
    "regex": "^[a-zA-Z][a-zA-Z0-9_]{4,15}$",
    "example": "abc_123",
    "description": "字母开头，5-16位"
  },
  {
    "usage": "密码（字母开头，6-18位）",
    "regex": "^[a-zA-Z]\\w{5,17}$",
    "example": "a12345_",
    "description": "字母开头，6-18位"
  },
  {
    "usage": "强密码（无特殊字符）",
    "regex": "^(?=.*\\d)(?=.*[a-z])(?=.*[A-Z])[a-zA-Z0-9]{8,10}$",
    "example": "Abc12345",
    "description": "8-10位，含大小写字母和数字"
  },
  {
    "usage": "强密码（可含特殊字符）",
    "regex": "^(?=.*\\d)(?=.*[a-z])(?=.*[A-Z]).{8,10}$",
    "example": "Abc12345!",
    "description": "8-10位，含大小写字母、数字、特殊字符"
  },
  {
    "usage": "日期格式",
    "regex": "^\\d{4}-\\d{1,2}-\\d{1,2}$",
    "example": "2023-01-01",
    "description": "yyyy-mm-dd"
  },
  {
    "usage": "一年12个月",
    "regex": "^(0?[1-9]|1[0-2])$",
    "example": "01, 12",
    "description": "01-12或1-12"
  },
  {
    "usage": "一个月31天",
    "regex": "^((0?[1-9])|((1|2)[0-9])|30|31)$",
    "example": "01, 31",
    "description": "01-31或1-31"
  },
  {
    "usage": "钱的输入（整数）",
    "regex": "^(0|[1-9][0-9]*)$",
    "example": "10000",
    "description": "非负整数"
  },
  {
    "usage": "钱的输入（可负数）",
    "regex": "^(0|-?[1-9][0-9]*)$",
    "example": "-10000",
    "description": "允许负数"
  },
  {
    "usage": "钱的输入（小数）",
    "regex": "^[0-9]+(\\.[0-9]{1,2})?$",
    "example": "10000.00",
    "description": "最多两位小数"
  },
  {
    "usage": "钱的输入（带千分位）",
    "regex": "^([0-9]+|[0-9]{1,3}(,[0-9]{3})*)(\\.[0-9]{1,2})?$",
    "example": "10,000.00",
    "description": "千分位支持"
  },
  {
    "usage": "xml文件",
    "regex": "^([a-zA-Z]+-?)+[a-zA-Z0-9]+\\.[xX][mM][lL]$",
    "example": "test-1.xml",
    "description": "xml文件名"
  },
  {
    "usage": "中文字符",
    "regex": "[\\u4e00-\\u9fa5]",
    "example": "汉",
    "description": "单个中文字符"
  },
  {
    "usage": "双字节字符",
    "regex": "[^\\x00-\\xff]",
    "example": "汉字，日文",
    "description": "非ASCII字符"
  },
  {
    "usage": "空白行",
    "regex": "^\\s*$",
    "example": "（空行）",
    "description": "匹配空白行"
  },
  {
    "usage": "HTML标记",
    "regex": "<[^>]+>",
    "example": "<div>",
    "description": "简单HTML标签"
  },
  {
    "usage": "首尾空白字符",
    "regex": "^\\s*|\\s*$",
    "example": "abc",
    "description": "去除首尾空白"
  },
  {
    "usage": "腾讯QQ号",
    "regex": "[1-9][0-9]{4,}",
    "example": "10000",
    "description": "5位及以上数字"
  },
  {
    "usage": "中国邮政编码",
    "regex": "[1-9]\\d{5}(?!\\d)",
    "example": "100000",
    "description": "6位数字"
  },
  {
    "usage": "IPv4地址",
    "regex": "((2(5[0-5]|[0-4]\\d))|[0-1]?\\d{1,2})(\\.((2(5[0-5]|[0-4]\\d))|[0-1]?\\d{1,2})){3}",
    "example": "192.168.1.1",
    "description": "IPv4格式"
  }
]

const regexInput = ref('')
const appendText = ref('/g')
const globalSearch = ref(true)
const ignoreCase = ref(false)
const multiline = ref(false)
const containLineBreak = ref(false)

const srcStr = ref('')
const matchResult = ref()
const matchResultStr = ref('')

const handleGlobalChange = (value: string) => {
  if (!value) {
    appendText.value = appendText.value.replace(/g/g, '');
  } else {
    appendText.value = appendText.value += 'g'
  }
}
const handleIgnoreCase = (value: string) => {
  if (!value) {
    appendText.value = appendText.value.replace(/i/g, '');
  } else {
    appendText.value = appendText.value += 'i'
  }
}
const handleMultiLine = (value: string) => {
  if (!value) {
    appendText.value = appendText.value.replace(/m/g, '');
  } else {
    appendText.value = appendText.value += 'm'
  }
}
const handleLineBreak = (value: string) => {
  if (!value) {
    appendText.value = appendText.value.replace(/s/g, '');
  } else {
    appendText.value = appendText.value += 's'
  }
}

const getMatchResult = () => {
  if (!regexInput.value) {
    ElMessage.error('请输入正则表达式');
    return;
  }
  if (!srcStr.value) {
    ElMessage.error('请输入要匹配的字符串');
    return;
  }
  const regex = new RegExp(regexInput.value, appendText.value.replace(/\//g, ''));
  matchResult.value = srcStr.value.match(regex);
  if (matchResult.value === null) {
    matchResultStr.value = ''
  } else {
    matchResultStr.value = `共找到${matchResult.value.length}处匹配`
  }
}
</script>

<template>

  <div class="general-box">
    <h2>正则表达式测试</h2>
    <div class="row">
      <el-input
          v-model="regexInput"
          style="max-width: 600px"
          placeholder="请输入正则表达式"
          class="input-with-select"
      >
        <template #prepend>
          /
        </template>
        <template #append>
          {{ appendText }}
        </template>
      </el-input>
      <div class="flex flex-wrap items-center">
        <el-dropdown>
          <el-button type="primary">
            修饰符<el-icon class="el-icon--right"><arrow-down /></el-icon>
          </el-button>
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item>
                <el-checkbox v-model="globalSearch" label="全局搜索-g" @change="handleGlobalChange"/>
              </el-dropdown-item>
              <el-dropdown-item>
                <el-checkbox v-model="ignoreCase" label="忽略大小写-i" @change="handleIgnoreCase"/>
              </el-dropdown-item>
              <el-dropdown-item>
                <el-checkbox v-model="multiline" label="多行模式-m" @change="handleMultiLine"/>
              </el-dropdown-item>
              <el-dropdown-item>
                <el-checkbox v-model="containLineBreak" label="包含换行符-s" @change="handleLineBreak"/>
              </el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
      </div>
    </div>
    <div style="margin-top: 12px;">
      <el-input
          v-model="srcStr"
          style="width: 100%;margin-top: 12px"
          :rows="2"
          type="textarea"
          placeholder="请输入要匹配的字符串"
      />
      <el-button type="primary" @click.stop="getMatchResult" style="margin-top: 12px;">开始匹配</el-button>
      <div class="match-result">
        <div>
          {{matchResultStr}}
        </div>
        <div v-for="(item, index) in matchResult" :key="index" class="match-result-item">
          {{item}}
        </div>
      </div>
    </div>
  </div>

  <div class="general-box">
    <h2>常用正则表达式</h2>
    <el-table :data="tableData" border style="width: 100%" :row-class-name="tableRowClassName">
      <el-table-column prop="usage" label="用途"/>
      <el-table-column prop="regex" label="正则表达式">
        <template #default="scope">
          <span>{{ scope.row.regex }}</span>
          <IconFont name="fuzhi" v-copy="scope.row.regex"/>
        </template>
      </el-table-column>
      <el-table-column prop="example" label="示例"/>
      <el-table-column prop="description" label="说明"/>
    </el-table>
  </div>
</template>

<style scoped>
.row {
  display: flex;
  align-items: center;
}

.example-showcase .el-dropdown-link {
  cursor: pointer;
  color: var(--el-color-primary);
  display: flex;
  align-items: center;
}

.match-result {
  width: 99%;
  height: 96px;
  padding-top: 2px;
  padding-bottom: 2px;
  padding-left: 8px;
  border: 1px solid #dcdfe6;
  border-radius: 4px;
  margin-top: 12px;
  overflow-y: auto;
}

.match-result::-webkit-scrollbar {
  display: none;
}


.match-result .match-result-item {
  margin: 3px 0;
}

:deep(.el-dropdown) {
  cursor: pointer;
  margin-left: 8px;
}
</style>

<style>
.el-table .warning-row {
  --el-table-tr-bg-color: #F2F2F2;
}

.el-table .success-row {
  --el-table-tr-bg-color: #FFFFFF;
}
</style>