import { useCallback, useEffect, useState } from "react";


type Props = {
  containerClassName?: string
  timeClassName?: string
  dateClassName?: string
}

export function DateTime(props: Props) {

  const [time, setTime] = useState(new Date());

  useEffect(() => {
    const timer = setInterval(() => {
      setTime(new Date());
    }, 1000);

    return () => clearInterval(timer);
  }, []);

  const formatTime = useCallback((date: Date) => {
    return date.toLocaleTimeString('en-US', {
      hour: '2-digit',
      minute: '2-digit',
      hour12: true
    });
  }, []);

  const formatDate = useCallback((date: Date) => {
    return date.toLocaleDateString('en-US', {
      weekday: 'short',
      month: 'short',
      day: 'numeric'
    });
  }, []);

  return (
    <div className={props.containerClassName}>
      <span className={props.timeClassName}> {formatTime(time)} </span>
      <span className={props.dateClassName}> {formatDate(time)} </span>
    </div>
  )

}
