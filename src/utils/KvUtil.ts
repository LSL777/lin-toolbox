export const getLabelByValue = (value: any, options: Array<{value: any, label: string}>) => {
    const target = options.find(option => option.value === value);
    if (target) {
        return target.label;
    } else {
        return '未找到匹配值'
    }
}