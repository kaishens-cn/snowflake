import test from 'ava'

import { Snowflake } from '../index.js'

test('sum from native', (t) => {
  const snow = new Snowflake();
  const res = snow.nextId();
  t.pass();
})
