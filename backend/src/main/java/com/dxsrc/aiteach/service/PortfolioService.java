package com.dxsrc.aiteach.service;

import com.dxsrc.aiteach.entity.Portfolio;
import java.util.List;

public interface PortfolioService {

    List<Portfolio> list();

    Portfolio getById(Long id);

    boolean save(Portfolio entity);

    boolean update(Portfolio entity);

    boolean removeById(Long id);
}
