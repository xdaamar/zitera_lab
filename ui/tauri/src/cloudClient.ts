// ZITERA CYBER_LAB — Cloud Client (Native Enclave IPC Bridge)
// All database queries and authentication tokens are routed exclusively through
// the native Rust security enclave, completely scrubbing raw credentials from JavaScript.

import { invoke } from '@tauri-apps/api/core'

export interface CloudQueryResult {
  affectedRowCount: number
  columns: string[]
  rows: Record<string, unknown>[]
}

export async function cloudExecute(sql: string, args?: unknown[]): Promise<CloudQueryResult> {
  const results = await cloudBatch([{ sql, args }])
  return results[0]
}

export async function cloudBatch(
  statements: { sql: string; args?: unknown[] }[],
): Promise<CloudQueryResult[]> {
  try {
    const formattedStatements = statements.map((stmt) => ({
      sql: stmt.sql,
      args: stmt.args ?? [],
    }))
    return await invoke<CloudQueryResult[]>('cloud_execute_batch', {
      statements: formattedStatements,
    })
  } catch (err) {
    console.error('[SECURITY ENCLAVE] Cloud Enclave execution error:', err)
    throw new Error(`Enclave Database Error: ${err}`)
  }
}
