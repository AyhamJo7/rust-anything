import { execFileSync } from 'node:child_process'
import { writeFileSync } from 'node:fs'
import { join } from 'node:path'
import { WEBSITE_ROOT } from './md-include.mjs'

const SKIP = new Set(['claude'])

const out = execFileSync(
  'gh',
  ['api', '--paginate', 'repos/erkamyaman/rust-anything/contributors?per_page=100', '-q', '.[] | select(.type == "User") | [.login, .contributions] | @tsv'],
  { encoding: 'utf8' },
)
const people = out
  .trim()
  .split('\n')
  .map((l) => l.split('\t'))
  .filter(([login]) => login && !SKIP.has(login.toLowerCase()))
  .map(([login, commits]) => ({ login, commits: Number(commits) }))

const file = join(WEBSITE_ROOT, 'content', 'contributors.json')
writeFileSync(file, JSON.stringify(people, null, 2) + '\n')
console.log(`contributors: ${people.length} -> ${file}`)
