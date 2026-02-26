import { Battery, BatteryCharging, BatteryFull, BatteryLow, BatteryMedium, Volume, Volume1, Volume2, VolumeX, Wifi, WifiOff } from "lucide-react";
import { useSystemStatus } from "../../hooks/useSystemStatus";
import { useCallback } from "react";
import { R_SystemStatus } from "../../types/types";


type Props = {
  containerClassName?: string
}

export function SystemStatus({ containerClassName }: Props) {

  const status = useSystemStatus();


  const getVolumeIcon = useCallback((status: R_SystemStatus) => {
    if (status.muted) return <VolumeX />
    if (status.volume_percent <= 30) return <Volume />
    if (status.volume_percent <= 80) return <Volume1 />
    return <Volume2 />
  }, [])

  const getBatteryIcon = useCallback((status: R_SystemStatus) => {
    if (status.battery_percent == null) return <></>
    if (status.charging) return <BatteryCharging />
    if (status.battery_percent <= 20) return <Battery />
    if (status.battery_percent <= 40) return <BatteryLow />
    if (status.battery_percent <= 90) return <BatteryMedium />
    return <BatteryFull />
  }, [])

  if (status == null) {
    return <div className={containerClassName}> ... </div>
  }

  return (
    <div className={containerClassName}>
      <div>
        {getVolumeIcon(status)}
        <span>{status.volume_percent}%</span>
      </div>
      <div>
        {
          status.wifi_up ? (
            <>
              <Wifi />
              <span>{status.ssid}</span>
            </>
          ) : <WifiOff />
        }
      </div>
      {
        status.battery_percent !== null ?
          (
            <div>
              {getBatteryIcon(status)}
              <span>{status.battery_percent}%</span>
            </div>
          ) : <> </>
      }
    </div>
  )
}
