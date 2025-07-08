import { CommandBus } from './CommandBus';
import { CommandLoggingMiddleware } from './middleware/command-logging-middleware';
import { Command, CommandMiddleware } from './types';

export type { Command, CommandMiddleware };

export const makeBus = (middleware: CommandMiddleware[] = []) =>
    new CommandBus([CommandLoggingMiddleware, ...middleware]);

export const appCommandBus = makeBus();

export type { CommandBus };

export {RenderHtmlCommand} from './RenderHtmlCommand';
