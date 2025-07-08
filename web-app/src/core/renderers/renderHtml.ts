import { h, ComponentType, Attributes } from 'preact';
import { renderToString } from 'preact-render-to-string';

import {AnyProps} from '@/core/stdlib/types';

export function renderHtml<P extends AnyProps = {}>(Component: ComponentType<P>, props?: P) {
    return renderToString(h(Component, props as Attributes & P));
}
