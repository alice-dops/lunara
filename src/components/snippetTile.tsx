import { DynamicIcon } from "lucide-react/dynamic"
import { SnippetViewDisplay } from "../types/types"
import Styles from './snippetTile.module.scss'

type Props = {
  snippet: SnippetViewDisplay
  isFocused?: boolean
  innerRef?: React.Ref<HTMLDivElement>;
}


export function SnippetTile({ snippet: snippet, isFocused, innerRef }: Props) {
  return (
    <div
      className={[Styles.tile, isFocused ? Styles.focus : ""].join(" ")}
      ref={innerRef}
    >
      <div className={Styles.snippet_icon}>
        <DynamicIcon name={snippet.display.icon} />
      </div>
      <div className={Styles.snippet_name_container}>
        <span className={Styles.snippet_name}>{snippet.display.name}</span>
      </div>
    </div>
  )
}
