<script setup>
import { ElMessage } from "element-plus";

const formatJSON = () => {
  const input = document.getElementById('jsonInput').value;
  try {
    const parsed = JSON.parse(input);
    const formatted = JSON.stringify(parsed, null, 2);
    document.getElementById('afterTreatment').value = formatted;
    showOutput("✅ JSON 已格式化", "success");
  } catch (e) {
    showOutput("❌ JSON 格式错误：" + e.message, "error");
  }
}

const minifyJSON = () => {
  const input = document.getElementById('jsonInput').value;
  try {
    const parsed = JSON.parse(input);
    const minified = JSON.stringify(parsed);
    document.getElementById('afterTreatment').value = minified;
    showOutput("✅ JSON 已压缩", "success");
  } catch (e) {
    showOutput("❌ JSON 格式错误：" + e.message, "error");
  }
}

const validateJSON = () => {
  const input = document.getElementById('jsonInput').value;
  try {
    JSON.parse(input);
    showOutput("✅ JSON 格式正确", "success");
  } catch (e) {
    showOutput("❌ JSON 格式错误：" + e.message, "error");
  }
}

const showOutput = (message, type = "") => {
  const outputDiv = document.getElementById('output');
  outputDiv.className = "";
  if (type) outputDiv.classList.add(type);
  outputDiv.textContent = message;
}

// 新增：将字符串转换为小写驼峰格式
const toCamelCase = (str) => {
  return str.replace(/[-_][a-z]/g, (match) =>
    match.charAt(1).toUpperCase())
    .replace(/^[A-Z]/, (match) => match.toLowerCase());
}

// 新增：将字符串转换为首字母大写的驼峰格式
const toPascalCase = (str) => {
  const camelCase = toCamelCase(str);
  return camelCase.charAt(0).toUpperCase() + camelCase.slice(1);
}

// 新增：转换JSON为Java Bean对象
const convertToJavaBean = () => {
  const input = document.getElementById('jsonInput').value;
  try {
    const parsed = JSON.parse(input);
    let javaCode = '';

    // 处理JSON对象
    if (typeof parsed === 'object' && !Array.isArray(parsed)) {
      javaCode = convertObjectToJavaClass(parsed, 'RootBean');
    }
    // 处理JSON数组
    else if (Array.isArray(parsed)) {
      javaCode = convertArrayToJavaClass(parsed);
    }
    // 处理其他类型
    else {
      javaCode = convertPrimitiveToJavaClass(parsed);
    }

    document.getElementById('afterTreatment').value = javaCode;
    showOutput("✅ 已生成 Java Bean 代码", "success");
  } catch (e) {
    showOutput("❌ 转换失败：" + e.message, "error");
  }
}

// 新增：转换对象为Java类
const convertObjectToJavaClass = (obj, className) => {
  let fields = '';
  let gettersSetters = '';
  let innerClasses = '';

  for (const key in obj) {
    if (obj.hasOwnProperty(key)) {
      const value = obj[key];
      const camelKey = toCamelCase(key);
      const pascalKey = toPascalCase(key);
      let fieldType = 'Object';
      let fieldDeclaration = '';

      // 根据值类型推断Java类型
      if (value === null) {
        fieldType = 'Object';
      } else if (Array.isArray(value)) {
        fieldType = convertArrayType(value, pascalKey);
      } else if (typeof value === 'object') {
        fieldType = pascalKey + 'Bean';
        innerClasses += '\n\n' + convertObjectToJavaClass(value, fieldType);
      } else {
        fieldType = getPrimitiveType(value);
      }

      // 生成字段声明
      fieldDeclaration = `    private ${fieldType} ${camelKey};`;
      fields += fieldDeclaration + '\n';

      // 生成getter和setter
      gettersSetters += `
    public ${fieldType} get${pascalKey}() {
        return this.${camelKey};
    }

    public void set${pascalKey}(${fieldType} ${camelKey}) {
        this.${camelKey} = ${camelKey};
    }\n`;
    }
  }

  // 组合成完整的Java类
  return `public class ${className} {${fields ? '\n' + fields : ''}
${gettersSetters}}${innerClasses}`;
}

// 新增：转换数组类型
const convertArrayType = (array, prefix) => {
  if (array.length === 0) return 'List<Object>';

  const firstElement = array[0];
  let elementType = 'Object';

  if (Array.isArray(firstElement)) {
    elementType = 'List<' + convertArrayType(firstElement, prefix) + '>';
  } else if (typeof firstElement === 'object' && firstElement !== null) {
    elementType = prefix + 'ItemBean';
  } else {
    elementType = getPrimitiveType(firstElement);
  }

  return 'List<' + elementType + '>';
}

// 新增：获取基本类型对应的Java类型
const getPrimitiveType = (value) => {
  if (typeof value === 'string') return 'String';
  if (typeof value === 'number') return Number.isInteger(value) ? 'Integer' : 'Double';
  if (typeof value === 'boolean') return 'Boolean';
  return 'Object';
}

// 新增：处理数组转换
const convertArrayToJavaClass = (array) => {
  if (array.length === 0) return '// 空数组无法推断类型';

  const firstItem = array[0];
  let itemType = 'Object';

  if (typeof firstItem === 'object' && firstItem !== null) {
    return convertObjectToJavaClass(firstItem, 'ItemBean');
  }

  return `public class ItemBean {
    private ${getPrimitiveType(firstItem)} value;
    
    public ${getPrimitiveType(firstItem)} getValue() {
        return value;
    }
    
    public void setValue(${getPrimitiveType(firstItem)} value) {
        this.value = value;
    }
}`;
}

// 新增：处理基本类型转换
const convertPrimitiveToJavaClass = (value) => {
  const type = getPrimitiveType(value);
  return `public class ValueBean {
      private ${type} value;
      
      public ${type} getValue() {
          return value;
      }
      
      public void setValue(${type} value) {
          this.value = value;
      }
  }`;
}
</script>

<template>
  <h2 style="text-align: left">JSON 工具</h2>

  <div class="text-region">
    <textarea id="jsonInput" placeholder="在此粘贴你的 JSON 内容..."></textarea>
    <textarea id="afterTreatment" placeholder="处理后的内容"></textarea>
    <button class="copy-btn minify-btn" v-copy="{ selector: '#afterTreatment' }">复制</button>
  </div>

  <div class="btn-group">
    <button class="format-btn" @click.stop="formatJSON()">格式化</button>
    <button class="minify-btn" @click.stop="minifyJSON()">压缩</button>
    <button class="validate-btn" @click.stop="validateJSON()">校验格式</button>
    <button class="convert-btn" @click.stop="convertToJavaBean()">转Java Bean</button>
  </div>

  <div id="output"></div>
</template>

<style scoped>
h2 {
  text-align: center;
}

textarea {
  flex: 1;
  height: 500px;
  padding: 10px;
  font-family: monospace;
  font-size: 14px;
  border: 1px solid #ccc;
  resize: vertical;
  box-sizing: border-box;
  margin-bottom: 10px;
}

.btn-group {
  display: flex;
  justify-content: center;
  gap: 10px;
  margin-bottom: 10px;
}

button {
  padding: 10px 20px;
  font-size: 14px;
  cursor: pointer;
  border: none;
  border-radius: 4px;
  transition: background 0.3s ease;
}

button:hover {
  opacity: 0.9;
}

.format-btn {
  background-color: #4caf50;
  color: white;
}

.minify-btn {
  background-color: #2196f3;
  color: white;
}

.validate-btn {
  background-color: #ff9800;
  color: white;
}

.convert-btn {
  background-color: #9c27b0;
  color: white;
}

#output {
  background-color: #fff;
  border: 1px solid #ccc;
  padding: 10px;
  white-space: pre-wrap;
  word-wrap: break-word;
  min-height: 100px;
  margin-top: 10px;
}

.success {
  color: green;
}

.error {
  color: red;
}

.text-region {
  display: flex;
  position: relative;
}

.text-region>textarea:first-of-type {
  margin-right: 8px;
}

.copy-btn {
  position: absolute;
  right: 4px;
  top: 4px;
}
</style>