/**
 * @param {string} s
 * @return {boolean}
 */
const isNumber = s => /^[+-]?([0-9]+(\.)?|[0-9]*\.[0-9]+)([eE][+-]?[0-9]+)?$/.test(s);
