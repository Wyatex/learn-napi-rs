import test from 'ava'

import { aFunction, coolFunction } from '../index.js'

test('aFunction', t => {
  t.is(aFunction(1), 2)
})

test('coolFunction', t => {
  t.is(coolFunction(1), 2)
})