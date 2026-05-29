package com.dxsrc.aiteach.service;

import com.dxsrc.aiteach.entity.Mindmap;
import java.util.List;

public interface MindmapCollabService {

    List<Mindmap> list();

    Mindmap getById(Long id);

    boolean save(Mindmap entity);

    boolean update(Mindmap entity);

    boolean removeById(Long id);
}
