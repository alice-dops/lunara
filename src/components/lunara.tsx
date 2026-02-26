import { useCallback, useState } from 'react';
import { ApplicationsGrid } from './applicationsGrid'
import Styles from './lunara.module.scss'
import { SnippetsGrid } from './snippetsGrid'
import { StatusBar } from './statusBar'
import { NavDir, useLunaraInput } from '../contexts/lunarInputContext';


export function Lunara() {

  const [selected, setSelected] = useState<number>(0);

  const onDirection = useCallback((dir: NavDir) => {
    if (dir == "up" || dir == "down") setSelected((s) => s === 1 ? 0 : 1)
  }, [])

  useLunaraInput({
    onDirection: onDirection
  })

  return (
    <div className={Styles.root_container}>
      <div className={Styles.container}>
        <StatusBar />
        <ApplicationsGrid focused={selected === 0} />
        <SnippetsGrid focused={selected === 1} />
      </div>
    </div>
  )
}
