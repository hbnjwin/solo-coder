package com.dxsrc.aiteach.service;

import com.dxsrc.aiteach.entity.Competition;
import java.util.List;

public interface CompetitionAuditService {

    List<Competition> list();

    Competition getById(Long id);

    boolean save(Competition entity);

    boolean update(Competition entity);

    boolean removeById(Long id);
}
