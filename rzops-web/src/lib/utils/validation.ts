/**
 * 表单验证工具
 */

/**
 * 验证日期范围：开始日期必须早于结束日期
 * @returns 错误信息，验证通过返回 null
 */
export function validateDateRange(
  startDate: string | undefined | null,
  endDate: string | undefined | null,
  startLabel = '开始日期',
  endLabel = '结束日期'
): string | null {
  if (!startDate || !endDate) return null;
  const start = new Date(startDate);
  const end = new Date(endDate);
  if (isNaN(start.getTime()) || isNaN(end.getTime())) return null;
  if (start > end) {
    return `${startLabel}不能晚于${endLabel}`;
  }
  return null;
}
