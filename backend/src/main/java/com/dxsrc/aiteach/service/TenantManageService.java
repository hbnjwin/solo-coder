package com.dxsrc.aiteach.service;

import com.dxsrc.aiteach.entity.Tenant;
import java.util.List;

public interface TenantManageService {

    List<Tenant> list();

    Tenant getById(Long id);

    boolean save(Tenant entity);

    boolean update(Tenant entity);

    boolean removeById(Long id);
}
