<script setup>

const formatJSON = () => {
  const input = document.getElementById('jsonInput').value;
  try {
    const parsed = JSON.parse(input);
    document.getElementById('afterTreatment').value = JSON.stringify(parsed, null, 2);
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


function capitalize (str) {
  return str.charAt(0).toUpperCase() + str.slice(1);
}

function jsTypeToJava (val) {
  const t = typeof val;
  if (t === "number") return Number.isInteger(val) ? "Integer" : "Double";
  if (t === "string") return "String";
  if (t === "boolean") return "Boolean";
  return "Object";
}

function generate () {
  const jsonStr = document.getElementById("jsonInputConvert").value.trim();
  const className = capitalize(document.getElementById("className").value.trim()) || "Root";
  const useLombok = document.getElementById("useLombok").checked;

  let json;
  try {
    json = JSON.parse(jsonStr);
  } catch (e) {
    alert("请输入有效的 JSON！");
    return;
  }

  const classes = {};
  function parseObject (obj, name) {
    if (classes[name]) return;
    const fields = [];
    for (const key in obj) {
      const val = obj[key];
      let type = "Object";
      if (typeof val === "number") {
        type = Number.isInteger(val) ? "Integer" : "Double";
      } else if (typeof val === "string") {
        type = "String";
      } else if (typeof val === "boolean") {
        type = "Boolean";
      } else if (Array.isArray(val)) {
        if (val.length > 0) {
          const first = val[0];
          const itemType = typeof first === "object"
              ? capitalize(key) + "Item"
              : jsTypeToJava(first);
          if (typeof first === "object") parseObject(first, itemType);
          type = `List<${itemType}>`;
        } else {
          type = "List<Object>";
        }
      } else if (typeof val === "object" && val !== null) {
        const subName = capitalize(key);
        parseObject(val, subName);
        type = subName;
      }
      fields.push({ key, type });
    }
    classes[name] = fields;
  }

  parseObject(json, className);

  let code = "";
  const importLombok = useLombok ? "import lombok.Data;\n" : "";
  const importList = Object.values(classes).some(f => f.some(v => v.type.startsWith("List<")))
      ? "import java.util.List;\n"
      : "";

  for (const [cls, fields] of Object.entries(classes).reverse()) {
    code += useLombok ? "@Data\n" : "";
    code += `public class ${cls} {\n`;
    fields.forEach(f => {
      code += `    private ${f.type} ${f.key};\n`;
    });
    if (!useLombok) {
      code += "\n";
      fields.forEach(f => {
        const capKey = capitalize(f.key);
        code += `    public ${f.type} get${capKey}() { return ${f.key}; }\n`;
        code += `    public void set${capKey}(${f.type} ${f.key}) { this.${f.key} = ${f.key}; }\n`;
      });
    }
    code += "}\n\n";
  }

  document.getElementById("convertOutput").textContent =
      importLombok + importList + "\n" + code.trim();
}

function copyCode () {
  const code = document.getElementById("convertOutput").textContent;
  navigator.clipboard.writeText(code).then(() => {
    alert("代码已复制到剪贴板！");
  });
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
  </div>

  <div id="output"></div>

  <el-divider />

  <h2>🧩 JSON 转 Java 类生成器</h2>

  <label>类名（首字母大写）：
    <input type="text" id="className" value="Root" />
  </label>
  <label style="margin-left: 20px;">
    <input type="checkbox" id="useLombok" />
    使用 @Data (Lombok)
  </label>
  <br />

  <textarea id="jsonInputConvert" placeholder="请输入 JSON 数据..."></textarea>
  <br />
  <button class="btn" @click.stop="generate">生成 Java 类代码</button>
  <button class="btn" @click.stop="copyCode()">复制代码</button>

  <h3>Java 类代码：</h3>
  <pre id="convertOutput"></pre>
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

button:hover {
  opacity: 0.9;
}

.format-btn {
  padding: 10px 20px;
  font-size: 14px;
  cursor: pointer;
  border: none;
  border-radius: 4px;
  transition: background 0.3s ease;
  background-color: #4caf50;
  color: white;
}

.minify-btn {
  padding: 10px 20px;
  font-size: 14px;
  cursor: pointer;
  border: none;
  border-radius: 4px;
  transition: background 0.3s ease;
  background-color: #2196f3;
  color: white;
}

.validate-btn {
  padding: 10px 20px;
  font-size: 14px;
  cursor: pointer;
  border: none;
  border-radius: 4px;
  transition: background 0.3s ease;
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


#jsonInputConvert {
  width: 100%;
  height: 500px;
  font-family: monospace;
  font-size: 14px;
  margin-top: 12px;
}

pre {
  background: #1e1e1e;
  color: #dcdcdc;
  padding: 16px;
  overflow-x: auto;
  white-space: pre-wrap;
  border-radius: 6px;
}

.btn {
  margin: 10px 12px 10px 0;
  padding: 6px 12px;
  cursor: pointer;
}
</style>