import { useCallback, useEffect, useRef, useState } from "react";
import { useApps } from "../hooks/useApps";
import Styles from './applicationsGrid.module.scss'
import { ApplicationTile } from "./applicationTile";
import { runApp } from "../services/apps";
import { ActionBtn, NavDir, useLunaraInput } from "../contexts/lunarInputContext";

type Props = {
  focused: boolean
}

export function ApplicationsGrid({ focused }: Props) {

  let { apps } = useApps()

  const [selected, setSelected] = useState<number>(0);
  const imFocusedRef = useRef<boolean>(false);
  const gridRef = useRef<HTMLDivElement>(null);
  const focusedRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const container = gridRef.current;
    const focused = focusedRef.current;

    if (!container || !focused) return;

    const containerRect = container.getBoundingClientRect();
    const focusedRect = focused.getBoundingClientRect();

    const offset =
      focusedRect.left -
      containerRect.left -
      containerRect.width / 2 +
      focusedRect.width / 2;

    container.scrollBy({
      left: offset,
      behavior: "smooth"
    });

  }, [selected]);

  const onDirection = useCallback((dir: NavDir) => {
    if (!imFocusedRef.current) return;
    if (dir === "right" || dir === "left") {
      setSelected((c) => {
        if (dir === "right") return Math.min(apps.length - 1, c + 1);
        return Math.max(0, c - 1);
      });
    }
  }, [apps]);

  const onButton = useCallback((btn: ActionBtn) => {
    if (!imFocusedRef.current) return;
    if (btn == "A") {
      if (apps[selected]) {
        runApp(apps[selected].app.id)
      }
    }
  }, [apps, selected])

  useEffect(() => {
    imFocusedRef.current = focused;
  }, [focused])

  useLunaraInput({
    onDirection: onDirection,
    onAction: onButton,
  })

  return (
    <div className={Styles.container}>
      <div>
        <div style={{ marginBottom: '1rem' }}>
          <h2 style={{
            color: 'rgba(255, 255, 255, 0.6)',
            fontSize: '1rem',
            textTransform: 'uppercase',
            letterSpacing: '0.05em',
            marginBottom: '0.25rem',
            fontWeight: 400
          }}>
            All Applications
          </h2>
          <div style={{
            height: '0.25rem',
            width: '6rem',
            background: 'rgba(255, 255, 255, 0.3)',
            borderRadius: '9999px'
          }} />
        </div>
      </div>
      <div className={Styles.grid} ref={gridRef}>
        {apps.map((app, index) => (
          <ApplicationTile
            key={index}
            app={app}
            isFocused={focused && selected === index}
            innerRef={focused && selected === index ? focusedRef : undefined}
          />
        ))}
      </div>
    </div>
  )

}
