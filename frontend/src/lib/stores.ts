import { writable } from 'svelte/store'
import type { WhoResponse } from '../models/schemas'

export const user = writable<WhoResponse | undefined>(undefined)
