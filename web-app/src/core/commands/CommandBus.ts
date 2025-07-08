import { Command, CommandMiddleware } from './types';
import { withCommandContext } from './context';

// убрать тему с replaceWith
export class CommandBus {
    constructor(private readonly middleware: CommandMiddleware[] = []) {}

    async execute<I, O>(command: Command<I, O>, input: I): Promise<O> {
        const ref = { command };

        const invoke = () => ref.command.execute(input);

        const name = command.name;

        const chain = this.middleware.reduceRight(
            (next, mw) => () =>
                mw(ref.command, input, (replaceWith?: Command<I, O>) => {
                    if (replaceWith) ref.command = replaceWith;
                    return next();
                }),
            invoke,
        );

        return withCommandContext(ref.command.name, chain);
    }
}
