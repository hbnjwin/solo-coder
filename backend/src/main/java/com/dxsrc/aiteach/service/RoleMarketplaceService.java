package com.dxsrc.aiteach.service;

import com.dxsrc.aiteach.entity.AiRole;
import java.util.List;

public interface RoleMarketplaceService {

    List<AiRole> list();

    AiRole getById(Long id);

    boolean save(AiRole entity);

    boolean update(AiRole entity);

    boolean removeById(Long id);
}
