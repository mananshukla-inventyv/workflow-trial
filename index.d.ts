/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface TikvConnParams {
  tlsclusterenabled: boolean
  sslcacerti: string
  sslclientcerti: string
  sslclientkeycerti: string
  host: string
}
export interface BatchResponse {
  keys: Array<string>
  values?: Array<string>
}
export function getNextKey(): Promise<string>
export function initClient(tikvConnParam?: TikvConnParams | undefined | null): Promise<string>
export function startLogger(): void
export function getDocument(key: string, withCas: boolean, projectName?: string | undefined | null): Promise<any>
export function addDocument(
  key: string,
  value: any,
  projectName: string | undefined | null,
  updateInEs: boolean,
  retry?: number | undefined | null,
): Promise<string>
export function replaceDocument(
  key: string,
  value: any,
  cas: string | undefined | null,
  projectName: string | undefined | null,
  updateInEs: boolean,
  retry?: number | undefined | null,
): Promise<string>
export function getBatchUsingScan(
  start: string,
  end: string,
  batchSize: number,
  keysOnly: boolean,
  projectName?: string | undefined | null,
): Promise<BatchResponse>
export function getBatch(keys: Array<string>, projectName?: string | undefined | null): Promise<BatchResponse>
export function deleteSingleRecord(key: string, projectName?: string | undefined | null): Promise<string>
