import { useCallback } from "react"
import { R_AppView } from "../types/types"
import Styles from './applicationTile.module.scss'



type Props = {
  app: R_AppView
  isFocused?: boolean
  onSelect?: (id: string) => void
  innerRef?: React.Ref<HTMLDivElement>;
}

export function ApplicationTile({ app, isFocused, onSelect, innerRef }: Props) {

  const clickCallback = useCallback(() => {
    if (!onSelect) return;
    onSelect(app.app.id)
  }, [app.app.id, onSelect])

  return (
    <div
      className={[Styles.tile, isFocused ? Styles.focus : ""].join(" ")}
      ref={innerRef}
    >
      {app.app.icon ? (
        <div
          onClick={clickCallback}
          style={{
            position: 'absolute',
            inset: 0,
            backgroundImage: `url(${app.app.icon})`,
            backgroundSize: '100% auto',
            backgroundRepeat: 'no-repeat',
            backgroundPositionY: 'center'
          }} />
      ) : <></>}
      <div className={Styles.g_gradiant} />
      <div className={Styles.content}>
        <div className={Styles.title}>
          <span> {app.app.name} </span>
        </div>
      </div>
    </div>

  )
}
