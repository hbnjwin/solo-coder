package com.dxsrc.aiteach.service;

import com.dxsrc.aiteach.entity.Resource;
import java.util.List;

public interface ResourceShareService {

    List<Resource> list();

    Resource getById(Long id);

    boolean save(Resource entity);

    boolean update(Resource entity);

    boolean removeById(Long id);
}
