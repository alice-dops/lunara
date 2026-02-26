import { DateTime, SystemStatus } from './brain';
import Styles from './statusBar.module.scss'
import { User } from 'lucide-react';

export function StatusBar() {

  return (
    <div className={Styles.container}>
      <div className={Styles.user}>
        <div className={Styles.user_icon}>
          <User style={{ width: '1.75rem', height: '1.75rem', color: 'white' }} />
        </div>
        <span className={Styles.user_name}>Alice & Mélodie</span>
      </div>
      <DateTime
        containerClassName={Styles.datetime}
        dateClassName={Styles.date}
        timeClassName={Styles.time}
      />
      <SystemStatus containerClassName={Styles.status} />
    </div>
  )
}
