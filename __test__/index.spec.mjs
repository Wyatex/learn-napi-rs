import test from 'ava'

import { sum, DEFAULT_CONST, Animal, Kind } from '../index.js'

test('sum from native', (t) => {
  t.is(sum(1, 2), 3)
})

test('const from native', (t) => {
  t.is(DEFAULT_CONST, 12)
})

test('class from native', (t)=> {
  const cat = new Animal('cat', Kind.Cat)
  t.is(cat.name, 'cat')
  t.is(cat.kind, Kind.Cat)
})
