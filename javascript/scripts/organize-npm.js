const fs = require('fs');
const path = require('path');

const pkg = JSON.parse(fs.readFileSync('package.json', 'utf8'));
const version = process.env.RELEASE_VERSION || pkg.version;

const platforms = {
    'darwin-arm64': { os: 'darwin', cpu: 'arm64', artifact: 'aarch64-apple-darwin/graphbit-js.darwin-arm64.node' },
    'darwin-x64': { os: 'darwin', cpu: 'x64', artifact: 'x86_64-apple-darwin/graphbit-js.darwin-x64.node' },
    'linux-arm64-gnu': { os: 'linux', cpu: 'arm64', artifact: 'aarch64-unknown-linux-gnu/graphbit-js.linux-arm64-gnu.node' },
    'linux-x64-gnu': { os: 'linux', cpu: 'x64', artifact: 'x86_64-unknown-linux-gnu/graphbit-js.linux-x64-gnu.node' },
    'linux-x64-musl': { os: 'linux', cpu: 'x64', libc: 'musl', artifact: 'x86_64-unknown-linux-musl/graphbit-js.linux-x64-musl.node' },
    'win32-arm64-msvc': { os: 'win32', cpu: 'arm64', artifact: 'aarch64-pc-windows-msvc/graphbit-js.win32-arm64-msvc.node' },
    'win32-x64-msvc': { os: 'win32', cpu: 'x64', artifact: 'x86_64-pc-windows-msvc/graphbit-js.win32-x64-msvc.node' }
};

console.log(`🚀 Starting manual organization for version ${version}...`);

if (!fs.existsSync('npm')) fs.mkdirSync('npm');

for (const [platform, config] of Object.entries(platforms)) {
    const dir = path.join('npm', platform);
    if (!fs.existsSync(dir)) fs.mkdirSync(dir, { recursive: true });

    const binaryName = path.basename(config.artifact);
    const src = path.join('artifacts', config.artifact);
    const dst = path.join(dir, binaryName);

    if (fs.existsSync(src)) {
        console.log(`✅ Moving binary: ${src} -> ${dst}`);
        fs.renameSync(src, dst);
    } else {
        console.error(`❌ Error: Artifact not found at ${src}`);
        process.exit(1);
    }

    const platformPkg = {
        name: `@infinitibit_gmbh/graphbit-${platform}`,
        version: version,
        os: [config.os],
        cpu: [config.cpu],
        main: binaryName,
        files: [binaryName],
        license: "SEE LICENSE IN ../LICENSE.md",
        publishConfig: {
            access: "public"
        },
        repository: pkg.repository,
        engines: pkg.engines
    };

    if (config.libc) {
        platformPkg.libc = [config.libc];
    }

    fs.writeFileSync(path.join(dir, 'package.json'), JSON.stringify(platformPkg, null, 2) + '\n');
    console.log(`✅ Generated manifest: ${dir}/package.json`);
}

console.log('✨ Organization complete!');
