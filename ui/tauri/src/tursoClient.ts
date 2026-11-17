// Turso LibSQL HTTP Pipeline v2 Client for ZiteraLab React/Tauri frontend

const TURSO_URL = 'https://ziteralab-xdaamar.aws-ap-northeast-1.turso.io/v2/pipeline'
const TURSO_TOKEN =
  'eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCJ9.eyJhIjoicnciLCJpYXQiOjE3OTQ4MzAxMTMsImlkIjoiMDFhMWU0MzUtYmMwMS03OTk0LWFmZjctMDhhYmM5ODZhY2MyIiwia2lkIjoiUGlqYXZ5dzQwbVB0cTVUdnQyMzd6Q29ZRHpqQ0JTa3p3WVdVdGtGTnhqayIsInJpZCI6IjFmZmIyZjFhLTM0MmEtNDAwMS1hNTZmLTdiYmM1OGJlMzExZSJ9.Y-tIm1M0tlUAEzPIJXC7w8PRsh016Y8sSpbE24EoBzjJX9z9p1AJXK2rYTFYgILBvZvzGRmNy_0n2JqVxv6OCw'

export interface TursoQueryResult {
  affectedRowCount: number
  columns: string[]
  rows: Record<string, unknown>[]
}

function encodeArg(val: unknown): Record<string, string> {
  if (val === null || val === undefined) return { type: 'null' }
  if (typeof val === 'boolean') return { type: 'integer', value: val ? '1' : '0' }
  if (typeof val === 'number') {
    if (Number.isInteger(val)) return { type: 'integer', value: String(val) }
    return { type: 'float', value: String(val) }
  }
  return { type: 'text', value: String(val) }
}

function decodeValue(cell: Record<string, unknown> | null): unknown {
  if (!cell) return null
  const { type, value } = cell as { type?: string; value?: unknown }
  if (type === 'null' || value === undefined || value === null) return null
  if (type === 'integer') return typeof value === 'number' ? value : parseInt(String(value), 10)
  if (type === 'float') return typeof value === 'number' ? value : parseFloat(String(value))
  return String(value)
}

export async function tursoExecute(sql: string, args?: unknown[]): Promise<TursoQueryResult> {
  const results = await tursoBatch([{ sql, args }])
  return results[0]
}

export async function tursoBatch(
  statements: { sql: string; args?: unknown[] }[],
): Promise<TursoQueryResult[]> {
  const requests = statements.map((stmt) => {
    const stmtPayload: Record<string, unknown> = { sql: stmt.sql }
    if (stmt.args && stmt.args.length > 0) {
      stmtPayload.args = stmt.args.map(encodeArg)
    }
    return { type: 'execute', stmt: stmtPayload }
  })
  requests.push({ type: 'close' } as unknown as (typeof requests)[0])

  const resp = await fetch(TURSO_URL, {
    method: 'POST',
    headers: {
      Authorization: `Bearer ${TURSO_TOKEN}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ requests }),
  })

  if (!resp.ok) {
    const text = await resp.text()
    throw new Error(`Turso HTTP ${resp.status}: ${text}`)
  }

  const decoded = (await resp.json()) as { results: unknown[] }
  const rawResults = decoded.results ?? []

  return statements.map((_, i) => {
    const resItem = rawResults[i] as Record<string, unknown>
    if (resItem?.type === 'error') {
      const err = resItem.error as Record<string, string>
      throw new Error(`Turso SQL Error: ${err?.message ?? 'Unknown error'}`)
    }
    const responseObj = (resItem?.response as Record<string, unknown>) ?? {}
    const resultObj = (responseObj.result as Record<string, unknown>) ?? {}
    const colsList = (resultObj.cols as Array<Record<string, string>>) ?? []
    const colNames = colsList.map((c) => c.name ?? '')
    const affectedRowCount = (resultObj.affected_row_count as number) ?? 0
    const rawRows = (resultObj.rows as Array<Array<Record<string, unknown>>>) ?? []

    const rows = rawRows.map((cells) => {
      const rowMap: Record<string, unknown> = {}
      colNames.forEach((col, j) => {
        rowMap[col] = decodeValue(cells[j] ?? null)
      })
      return rowMap
    })

    return { affectedRowCount, columns: colNames, rows }
  })
}
