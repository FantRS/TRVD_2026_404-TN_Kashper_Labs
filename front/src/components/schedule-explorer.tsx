"use client";

import { startTransition, useEffect, useState } from "react";

import { getAvailableSlots } from "@/lib/api";
import { AvailableSlot } from "@/lib/types";
import { formatDate, formatDateTime } from "@/lib/utils";

function makeRange(offsetDays: number) {
  const start = new Date();
  start.setHours(0, 0, 0, 0);
  start.setDate(start.getDate() + offsetDays);

  const end = new Date(start);
  end.setDate(end.getDate() + 4);
  end.setHours(23, 59, 59, 999);

  return {
    from: start.toISOString(),
    to: end.toISOString(),
  };
}

function groupSlots(slots: AvailableSlot[]) {
  return slots.reduce<Record<string, AvailableSlot[]>>((groups, slot) => {
    const key = slot.scheduled_at.slice(0, 10);
    groups[key] = groups[key] ? [...groups[key], slot] : [slot];
    return groups;
  }, {});
}

interface ScheduleExplorerProps {
  selectable?: boolean;
  selectedSlot?: string | null;
  onSelectSlot?: (slot: string) => void;
}

export function ScheduleExplorer({
  selectable = false,
  selectedSlot = null,
  onSelectSlot,
}: ScheduleExplorerProps) {
  const [offsetDays, setOffsetDays] = useState(0);
  const [slots, setSlots] = useState<AvailableSlot[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;

    async function loadSlots() {
      setLoading(true);
      setError(null);

      try {
        const range = makeRange(offsetDays);
        const response = await getAvailableSlots(range.from, range.to);
        if (!cancelled) {
          setSlots(response);
        }
      } catch (loadError) {
        if (!cancelled) {
          setError(loadError instanceof Error ? loadError.message : "Не вдалося завантажити слоти.");
        }
      } finally {
        if (!cancelled) {
          setLoading(false);
        }
      }
    }

    void loadSlots();

    return () => {
      cancelled = true;
    };
  }, [offsetDays]);

  const grouped = groupSlots(slots);

  return (
    <div className="panel">
      <div className="section-heading compact">
        <div>
          <p className="eyebrow">Графік</p>
          <h3>Вільні вікна в календарі неминучого</h3>
        </div>
        <div className="inline-actions">
          <button
            type="button"
            className="button ghost"
            onClick={() => startTransition(() => setOffsetDays((current) => Math.max(0, current - 5)))}
            disabled={offsetDays === 0}
          >
            Назад у минуле
          </button>
          <button
            type="button"
            className="button ghost"
            onClick={() => startTransition(() => setOffsetDays((current) => current + 5))}
          >
            Ближче до фіналу
          </button>
        </div>
      </div>

      {error ? <p className="muted">{error}</p> : null}

      {loading ? (
        <div className="slots-grid">
          {Array.from({ length: 6 }).map((_, index) => (
            <div key={index} className="slot-button skeleton-card" />
          ))}
        </div>
      ) : (
        <div className="stack-24">
          {Object.entries(grouped).map(([day, daySlots]) => (
            <div key={day} className="day-slots">
              <div className="panel-header">
                <h4>{formatDate(day)}</h4>
                <span className="muted">{daySlots.filter((slot) => slot.is_available).length} шансів усе провести без тисняви</span>
              </div>
              <div className="slots-grid">
                {daySlots.map((slot) => {
                  const isSelected = selectedSlot === slot.scheduled_at;

                  return (
                    <button
                      key={slot.scheduled_at}
                      type="button"
                      className={
                        slot.is_available
                          ? isSelected
                            ? "slot-button active"
                            : "slot-button"
                          : "slot-button busy"
                      }
                      disabled={!slot.is_available || !selectable}
                      onClick={() => {
                        if (selectable && slot.is_available && onSelectSlot) {
                          onSelectSlot(slot.scheduled_at);
                        }
                      }}
                    >
                      <span>{formatDateTime(slot.scheduled_at)}</span>
                      <small>{slot.is_available ? "Можна займати" : "Тут уже хтось встиг раніше"}</small>
                    </button>
                  );
                })}
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
