package com.linkyoyo.tobacco.repository;

import com.cosium.spring.data.jpa.entity.graph.repository.EntityGraphJpaRepository;
import com.cosium.spring.data.jpa.entity.graph.repository.EntityGraphJpaSpecificationExecutor;
import com.linkyoyo.tobacco.entity.StandardSystem;

public interface StandardSystemRepository extends EntityGraphJpaRepository<StandardSystem, Integer>, EntityGraphJpaSpecificationExecutor<StandardSystem> {
}
