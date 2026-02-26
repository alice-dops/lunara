import { DynamicIcon } from "lucide-react/dynamic"
import { R_Snippet } from "../types/types"
import Styles from './snippetTile.module.scss'

type Props = {
  snippet: R_Snippet
  isFocused?: boolean
  innerRef?: React.Ref<HTMLDivElement>;
}

export function SnippetTile({ snippet: snipsset, isFocused, innerRef }: Props) {
  return (
    <div
      className={[Styles.tile, isFocused ? Styles.focus : ""].join(" ")}
      ref={innerRef}
    >
      <div className={Styles.snippet_icon}>
        <DynamicIcon name={snipsset.icon} />
      </div>
      <div className={Styles.snippet_name_container}>
        <span className={Styles.snippet_name}>{snipsset.name}</span>
      </div>
    </div>
  )
}
