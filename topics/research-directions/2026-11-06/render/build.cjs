const fs=require('fs'), path=require('path');
const katex=require('katex'), MarkdownIt=require('markdown-it');
const src=process.argv[2], dest=process.argv[3];
const formulas=[];
let source=fs.readFileSync(src,'utf8');
source=source.replace(/\$\$([\s\S]*?)\$\$|\$([^$\n]+)\$/g,(_,block,inline)=>{
 const i=formulas.length;
 formulas.push(katex.renderToString(block??inline,{displayMode:block!==undefined,throwOnError:true,strict:'error',output:'htmlAndMathml'}));
 return `MATHPLACEHOLDER${i}END`;
});
if(source.includes('$')) throw Error('Unmatched math delimiter');
let html=new MarkdownIt({html:false,typographer:true}).render(source);
html=html.replace(/<table>/g,'<div class="table-scroll"><table>').replace(/<\/table>/g,'</table></div>');
html=html.replace(/MATHPLACEHOLDER(\d+)END/g,(_,i)=>formulas[Number(i)]);
let css=fs.readFileSync(require.resolve('katex/dist/katex.min.css'),'utf8');
css=css.replace(/url\(([^)]+)\)/g,(_,url)=>{
 const font=path.join(path.dirname(require.resolve('katex/dist/katex.min.css')),url);
 const ext=path.extname(font).slice(1);
 return `url(data:font/${ext};base64,${fs.readFileSync(font).toString('base64')})`;
});
const toc=[]; let count=0;
html=html.replace(/<h2>(.*?)<\/h2>/g,(_,title)=>{const id=`section-${++count}`;toc.push(`<li><a href="#${id}">${title}</a></li>`);return `<h2 id="${id}">${title}</h2>`});
const style=`:root{color-scheme:light;--foreground:#202733;--background:#fff;--font-sans:system-ui,sans-serif;--muted:#586272;--primary:#195a89;--border:#d9dee5}body{color:var(--foreground);font:18px/1.65 Georgia,serif;margin:0}main{max-width:810px;margin:32px auto;padding:0 24px 60px}h1,h2,h3,nav{font-family:var(--font-sans)}h1{font-size:2rem;line-height:1.2}h2{font-size:1.4rem;line-height:1.3;margin-top:2.5em;scroll-margin-top:20px}h3{font-size:1.12rem;margin-top:1.8em}p{margin:1em 0}a{color:var(--primary);text-underline-offset:3px}.table-scroll{max-width:100%;overflow-x:auto;overscroll-behavior-inline:contain}table{border-collapse:collapse;min-width:640px}th,td{padding:8px;vertical-align:top;border:1px solid var(--border)}nav{font-size:.88rem;border-block:1px solid var(--border);padding:12px 0;margin:28px 0}nav ol{list-style:none;columns:2;padding-left:0;column-gap:32px}nav li{break-inside:avoid;margin:4px 0} .katex{font-size:1.08em}.katex-display{overflow-x:auto;overflow-y:hidden;padding:8px 0}li{margin:10px 0} @media(max-width:600px){body{font-size:17px}main{padding:0 14px}nav ol{columns:1}h1{font-size:1.6rem}}@media print{main{margin:0;max-width:none;padding:0}body{font-size:11pt}h2,h3{break-after:avoid}.katex-display{break-inside:avoid}a{color:inherit}}`;
const out=`<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Future research around computational symplectic geometry</title><style>${css}\n${style}</style></head><body><main>${html.replace(/(<\/h1>)/,`$1<nav aria-label="Contents"><details><summary>Contents</summary><ol>${toc.join('')}</ol></details></nav>`)}</main></body></html>`;
fs.writeFileSync(dest,out);
console.log(JSON.stringify({formulas:formulas.length,bytes:Buffer.byteLength(out),sections:count}));
