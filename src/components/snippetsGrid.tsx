import { useSnippets } from "../hooks/useSnippets"
import Styles from './snippetsGrid.module.scss'
import { useCallback, useEffect, useRef, useState } from "react";
import { ActionBtn, NavDir, useLunaraInput } from "../contexts/lunarInputContext";
import { SnippetTile } from "./snippetTile";
import { runSinppet } from "../services/apps";

type Props = {
  focused: boolean
}

export function SnippetsGrid({ focused }: Props) {

  const { snippets } = useSnippets()


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
        if (dir === "right") return Math.min(snippets.length - 1, c + 1);
        return Math.max(0, c - 1);
      });
    }
  }, [snippets]);

  // const onButton = useCallback((btn: ActionBtn) => {
  const onButton = useCallback((btn: ActionBtn) => {
    if (!imFocusedRef.current) return;
    if (snippets[selected]) {
      runSinppet(snippets[selected].id, btn);
    }

  }, [selected, snippets])


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
        <div style={{ marginBottom: '0.5rem' }}>
          <h2 style={{
            color: 'rgba(255, 255, 255, 0.6)',
            fontSize: '1rem',
            textTransform: 'uppercase',
            letterSpacing: '0.05em',
            marginBottom: '0.25rem',
            fontWeight: 400
          }}>
            All Snippets
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
        {snippets.map((snip, index) => (
          <SnippetTile
            key={index}
            snippet={snip}
            isFocused={focused && selected === index}
            innerRef={selected === index ? focusedRef : undefined}
          />
        ))}
      </div>
    </div>
  )
}
