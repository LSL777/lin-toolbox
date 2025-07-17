// directives/copy.js
import { ElMessage } from 'element-plus';
import { DirectiveBinding } from "vue";

export default {
    mounted(el: HTMLElement, binding: DirectiveBinding) {
        el.addEventListener('click', async () => {
            let textToCopy = '';

            // 判断绑定值的类型
            if (typeof binding.value === 'string') {
                // 情况1：直接传递字符串值
                textToCopy = binding.value;
            } else if (typeof binding.value === 'object') {
                if (binding.value.selector) {
                    // 情况2：通过选择器查找元素
                    const target = document.querySelector(binding.value.selector);
                    if (!target) {
                        ElMessage.error('未找到目标元素');
                        return;
                    }
                    // 优先使用 input/textarea 的 value，否则使用 innerText
                    textToCopy = 'value' in target ? target.value : target.innerText;
                } else if (binding.value.value !== undefined) {
                    // 情况3：通过 { value: '要复制的内容' } 传递值
                    textToCopy = binding.value.value;
                }
            }

            if (!textToCopy) {
                ElMessage.error('没有可复制的内容');
                return;
            }

            try {
                await navigator.clipboard.writeText(textToCopy);
                if (binding.value?.message !== false) {
                    ElMessage.success('复制成功');
                }
            } catch (err) {
                ElMessage.error('复制失败');
                console.error('复制失败:', err);
            }
        });
    },
};