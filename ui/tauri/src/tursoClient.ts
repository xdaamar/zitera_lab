// ZITERA CYBER_LAB — Turso Client (Native Enclave IPC Bridge)
// All database queries and authentication tokens are routed exclusively through
// the native Rust security enclave, completely scrubbing raw credentials from JavaScript.

import { invoke } from '@tauri-apps/api/core'

export interface TursoQueryResult {
  affectedRowCount: number
  columns: string[]
  rows: Record<string, unknown>[]
}

export async function tursoExecute(sql: string, args?: unknown[]): Promise<TursoQueryResult> {
  const results = await tursoBatch([{ sql, args }])
  return results[0]
}

export async function tursoBatch(
  statements: { sql: string; args?: unknown[] }[],
): Promise<TursoQueryResult[]> {
  try {
    const formattedStatements = statements.map((stmt) => ({
      sql: stmt.sql,
      args: stmt.args ?? [],
    }))
    return await invoke<TursoQueryResult[]>('turso_execute_batch', {
      statements: formattedStatements,
    })
  } catch (err) {
    console.error('[SECURITY ENCLAVE] Turso Enclave execution error:', err)
    throw new Error(`Enclave Database Error: ${err}`)
  }
}
