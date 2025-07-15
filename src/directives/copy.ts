// directives/copy.js
import { ElMessage } from 'element-plus';
import {DirectiveBinding} from "vue";

export default {
    mounted(el: HTMLElement, binding: DirectiveBinding) {
        el.addEventListener('click', async () => {
            const input = document.querySelector(binding.value.selector);
            if (!input) {
                ElMessage.error('未找到输入框');
                return;
            }

            try {
                await navigator.clipboard.writeText(input.value);
                if (binding.value.message !== false) {
                    ElMessage.success('复制成功');
                }
            } catch (err) {
                ElMessage.error('复制失败');
                console.error('复制失败:', err);
            }
        });
    },
};