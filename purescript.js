const html2json = require('html2json').html2json;
const fs = require('fs/promises');

let tags = []

async function fetch_html() {
  try {
    const data = await fs.readFile('./sample/index.html', { encoding: 'utf8' });
    return data;
  } catch (err) {
    console.log(err);
  }
}

async function save_purs(content) {
    try {
        const path = 'debug.json'
        const data = {
            data: content
        }
        await fs.unlinkSync(path)

        await fs.appendFile(path, JSON.stringify(data));
    } catch (err) {
        console.log(err);
    }
  }

function child_elems(str, child) {
    for(let elem of child) {
        switch(elem.node) {
            case 'element':
                str += elem.tag
                str = child_elems(str, elem.child)
                str += ','
                tags.push(elem.tag)
            break;
            case 'text':
                tags.push('text')
                let textLength = elem.text.replace(/ /g,"").length;
                if(textLength) str += `(${elem.node} "${elem.text}")`
            break;
        }    
    }
    return str
}

async function start() {
    const html = (await fetch_html()).replace(/\s+/g, ' ').trim();
    const json = html2json(html)

    let dom_elements = child_elems('', json.child[0].child)
    dom_elements = dom_elements.slice(0, -1)

    tags = tags.filter((v, i, a) => a.indexOf(v) === i);
    let TPS = tags.join()

    let pureScriptCode = `module Main where
    import Prelude
    import Data.Foldable (fold)
    import Effect (Effect)
    import TryPureScript (render,${TPS})
    main :: Effect Unit
    main =
      render $ fold
      [${dom_elements}]`;
    
    await save_purs(pureScriptCode)
}

start()