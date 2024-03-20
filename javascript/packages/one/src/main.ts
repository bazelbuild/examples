import isOdd from 'is-odd';

export function one() {
  return `I am One, not Two! Am I odd? ${isOdd(1)}`;
}
