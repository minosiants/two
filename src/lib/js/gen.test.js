import { expect, test } from 'vitest'
import s from './gen.js'


test('charGen', () => {
  expect(s.stream(s.charGen()).take(5).toArray()).toHaveLength(5);
});

test('stringGen', () => {
  expect(s.stream(s.stringGen(4)).take(5).toArray()).toHaveLength(5);
});
