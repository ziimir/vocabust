import { getRequestContext } from '../../context';
import { getCommandStack } from '../context';
import { CommandMiddleware } from '../types';

export const CommandLoggingMiddleware: CommandMiddleware = async (_command, input, next) => {
    const requestContext = getRequestContext();
    const requestId = requestContext.requestId;
    const commandStack = getCommandStack().join(' > ');

    console.log(`▶️ [${requestId}] Command: ${commandStack}`, input);
    const start = Date.now();

    try {
        const result = await next();
        console.log(`▶️ [${requestId}] Command: ${commandStack} ✅ took ${Date.now() - start}ms`);
        return result;
    } catch (err) {
        console.error(`▶️ [${requestId}] Command: ${commandStack} ❌ failed:`, err);
        throw err;
    }
};
