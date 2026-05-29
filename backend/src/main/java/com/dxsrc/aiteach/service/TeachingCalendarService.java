package com.dxsrc.aiteach.service;

import com.dxsrc.aiteach.entity.CalendarEvent;
import java.util.List;

public interface TeachingCalendarService {

    List<CalendarEvent> list();

    CalendarEvent getById(Long id);

    boolean save(CalendarEvent entity);

    boolean update(CalendarEvent entity);

    boolean removeById(Long id);
}
