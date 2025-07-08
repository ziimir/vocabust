import { AsyncLocalStorage } from 'node:async_hooks';

const store = new AsyncLocalStorage<string[]>();

export function withCommandContext<T>(commandName: string, fn: () => Promise<T>) {
    const prev = store.getStore() ?? [];
    const next = [...prev, commandName];
    return store.run(next, fn);
}

export function getCommandStack(): string[] {
    return store.getStore() ?? [];
}
