import { AsyncLocalStorage } from 'node:async_hooks';

export interface RequestContext {
    requestId: string;
    traceId?: string;
    userId?: string;
    locale?: string;
}

const store = new AsyncLocalStorage<RequestContext>();

export function withRequestContext<T>(ctx: RequestContext, fn: () => T) {
    return store.run(ctx, fn);
}

export function getRequestContext(): RequestContext {
    return store.getStore() ?? { requestId: 'unknown' };
}
