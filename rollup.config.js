import { nodeResolve } from '@rollup/plugin-node-resolve';

export default {
    input: 'app.js',
    output: {
        file: './dist/.stage/app.js',
        format: 'es'
    },
    plugins: [nodeResolve()]
};
