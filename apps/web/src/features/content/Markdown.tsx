import { Fragment } from 'react'

function inline(value: string) {
  return value.split(/(`[^`]+`|\*\*[^*]+\*\*)/g).map((part, index) => part.startsWith('`') ? <code key={index}>{part.slice(1, -1)}</code> : part.startsWith('**') ? <strong key={index}>{part.slice(2, -2)}</strong> : part)
}

/** Small shared renderer for Carbon's authored Markdown content. */
export function Markdown({ content }: { content: string }) {
  const lines = content.replace(/^---[\s\S]*?---\s*/, '').trim().split('\n')
  const nodes: React.ReactNode[] = []
  for (let index = 0; index < lines.length;) {
    const line = lines[index]
    if (line.startsWith('```')) { const code: string[] = []; while (++index < lines.length && !lines[index].startsWith('```')) code.push(lines[index]); nodes.push(<pre key={index}><code>{code.join('\n')}</code></pre>); index++; continue }
    const heading = line.match(/^(#{1,3})\s+(.+)/)
    if (heading) { const Tag = `h${heading[1].length}` as 'h1'; nodes.push(<Tag key={index}>{inline(heading[2])}</Tag>); index++; continue }
    if (line.includes('|') && lines[index + 1]?.match(/^\s*\|?\s*:?-+/)) {
      const cells = (row: string) => row.replace(/^\||\|$/g, '').split('|').map((cell) => cell.trim())
      const headers = cells(line); index += 2; const rows: string[][] = []
      while (index < lines.length && lines[index].includes('|')) rows.push(cells(lines[index++]))
      nodes.push(<table key={index}><thead><tr>{headers.map((cell, cellIndex) => <th key={cellIndex}>{inline(cell)}</th>)}</tr></thead><tbody>{rows.map((row, rowIndex) => <tr key={rowIndex}>{row.map((cell, cellIndex) => <td key={cellIndex}>{inline(cell)}</td>)}</tr>)}</tbody></table>); continue
    }
    if (/^- /.test(line)) { const items: string[] = []; while (index < lines.length && /^- /.test(lines[index])) items.push(lines[index++].slice(2)); nodes.push(<ul key={index}>{items.map((item, itemIndex) => <li key={itemIndex}>{inline(item)}</li>)}</ul>); continue }
    if (!line.trim()) { index++; continue }
    const paragraph: string[] = []; while (index < lines.length && lines[index].trim() && !/^#|^- |^```/.test(lines[index])) paragraph.push(lines[index++]); nodes.push(<p key={index}>{inline(paragraph.join(' '))}</p>)
  }
  return <>{nodes.map((node, index) => <Fragment key={index}>{node}</Fragment>)}</>
}
