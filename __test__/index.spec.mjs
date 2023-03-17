import test from 'ava'

import { Snowflake } from '../index'

test('generate id', (t) => {
  const snow = new Snowflake(1, 1);
  snow.nextId();
  t.pass();
})
