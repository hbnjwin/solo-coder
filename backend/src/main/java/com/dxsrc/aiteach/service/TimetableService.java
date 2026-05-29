package com.dxsrc.aiteach.service;

import com.dxsrc.aiteach.entity.Timetable;
import java.util.List;

public interface TimetableService {

    List<Timetable> list();

    Timetable getById(Long id);

    boolean save(Timetable entity);

    boolean update(Timetable entity);

    boolean removeById(Long id);
}
