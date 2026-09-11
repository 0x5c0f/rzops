/**
 * 表单验证工具
 */

/** 单条校验规则 */
export interface ValidationRule {
  /** 字段值 */
  value: unknown;
  /** 字段标签（用于错误提示） */
  label: string;
  /** 是否必填 */
  required?: boolean;
  /** 最小长度 */
  minLength?: number;
  /** 最大长度 */
  maxLength?: number;
  /** 最小值（数值） */
  min?: number;
  /** 最大值（数值） */
  max?: number;
  /** 格式校验类型 */
  format?: 'email' | 'url' | 'ip' | 'ipv4' | 'ipv6' | 'port' | 'domain' | 'positiveNumber';
  /** 自定义正则 */
  pattern?: RegExp;
  /** 自定义校验函数，返回错误信息或 null */
  custom?: (value: unknown) => string | null;
}

/**
 * 校验单个字段
 * @returns 错误信息，验证通过返回 null
 */
export function validateField(rule: ValidationRule): string | null {
  const { value, label, required, minLength, maxLength, min, max, format, pattern, custom } = rule;
  const strValue = value === undefined || value === null ? '' : String(value).trim();

  // 必填校验
  if (required && !strValue) {
    return `${label}不能为空`;
  }

  // 空值跳过后续校验（非必填）
  if (!strValue) return null;

  // 长度校验
  if (minLength !== undefined && strValue.length < minLength) {
    return `${label}至少需要${minLength}个字符`;
  }
  if (maxLength !== undefined && strValue.length > maxLength) {
    return `${label}不能超过${maxLength}个字符`;
  }

  // 格式校验
  if (format) {
    const formatError = validateFormat(strValue, format, label);
    if (formatError) return formatError;
  }

  // 自定义正则
  if (pattern && !pattern.test(strValue)) {
    return `${label}格式不正确`;
  }

  // 数值范围校验
  const numValue = Number(strValue);
  if (min !== undefined && !isNaN(numValue) && numValue < min) {
    return `${label}不能小于${min}`;
  }
  if (max !== undefined && !isNaN(numValue) && numValue > max) {
    return `${label}不能大于${max}`;
  }

  // 自定义校验
  if (custom) {
    return custom(value);
  }

  return null;
}

/**
 * 批量校验，返回第一个错误
 * @returns 第一个错误信息，全部通过返回 null
 */
export function validate(rules: ValidationRule[]): string | null {
  for (const rule of rules) {
    const error = validateField(rule);
    if (error) return error;
  }
  return null;
}

/**
 * 批量校验，返回所有错误
 * @returns 错误信息数组
 */
export function validateAll(rules: ValidationRule[]): string[] {
  return rules.map(validateField).filter((e): e is string => e !== null);
}

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

// ========== 格式校验 ==========

function validateFormat(value: string, format: string, label: string): string | null {
  switch (format) {
    case 'email':
      return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(value) ? null : `${label}格式不正确`;
    case 'url':
      return isValidUrl(value) ? null : `${label}格式不正确（需以 http:// 或 https:// 开头）`;
    case 'ip':
      return isIPv4(value) || isIPv6(value) ? null : `${label}格式不正确`;
    case 'ipv4':
      return isIPv4(value) ? null : `${label}不是有效的IPv4地址`;
    case 'ipv6':
      return isIPv6(value) ? null : `${label}不是有效的IPv6地址`;
    case 'port':
      return isValidPort(value) ? null : `${label}必须是1-65535之间的整数`;
    case 'domain':
      return isValidDomain(value) ? null : `${label}格式不正确`;
    case 'positiveNumber':
      return Number(value) > 0 ? null : `${label}必须是正数`;
    default:
      return null;
  }
}

function isIPv4(value: string): boolean {
  const parts = value.split('.');
  if (parts.length !== 4) return false;
  return parts.every(part => {
    const num = Number(part);
    return Number.isInteger(num) && num >= 0 && num <= 255 && part === String(num);
  });
}

function isIPv6(value: string): boolean {
  // 简化的IPv6校验
  const groups = value.split(':');
  if (groups.length < 2 || groups.length > 8) return false;
  // 允许 :: 缩写
  const hasDoubleColon = value.includes('::');
  if (!hasDoubleColon && groups.length !== 8) return false;
  return groups.every(group => {
    if (group === '') return true; // :: 缩写产生的空组
    return /^[0-9a-fA-F]{1,4}$/.test(group);
  });
}

function isValidPort(value: string): boolean {
  const num = Number(value);
  return Number.isInteger(num) && num >= 1 && num <= 65535;
}

function isValidUrl(value: string): boolean {
  try {
    const url = new URL(value);
    return url.protocol === 'http:' || url.protocol === 'https:';
  } catch {
    return false;
  }
}

function isValidDomain(value: string): boolean {
  // 域名格式：example.com / sub.example.com / example.co.uk
  return /^([a-zA-Z0-9]([a-zA-Z0-9-]*[a-zA-Z0-9])?\.)+[a-zA-Z]{2,}$/.test(value);
}
