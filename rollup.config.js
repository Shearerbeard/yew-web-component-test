import { nodeResolve } from '@rollup/plugin-node-resolve';

export default {
    input: 'app.js',
    output: {
        file: './dist/app.js',
        format: 'es'
    },
    plugins: [nodeResolve()]
};
