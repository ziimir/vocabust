import { ComponentType } from 'preact';

import {AnyProps} from '@/core/stdlib/types';
import { Command } from '@/core/commands';
import {renderHtml} from '@/core/renderers/renderHtml';

export class RenderHtmlCommand<P extends AnyProps = {}> implements Command<P, string> {
    constructor(private readonly component: ComponentType<P>) {}

    public readonly name = 'RenderHtmlCommand';

    async execute(props?: P): Promise<string> {
        return renderHtml(this.component, props);
    }
}
