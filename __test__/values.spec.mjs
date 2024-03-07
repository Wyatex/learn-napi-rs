import test from 'ava'

import { getNull, getEnv, getBufferSum, getUndefined, readBuffer } from '../index.js'


test('getNull', t => {
  t.is(getNull(), null)
})

test('getEnv', t => {
  const path = getEnv('PATH')
  t.is(!!path, true)
})

test('getBufferSum', t => {
  t.is(getBufferSum(Buffer.from([1,2,3])), 6)
})

test('getUndefined', t => {
  t.is(getUndefined(), undefined)
})

test('readBuffer', t => {
  t.is(readBuffer('./1.txt'), Buffer.from([31,32,33,61,62,63]))
})
