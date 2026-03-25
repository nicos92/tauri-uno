import { defineStore } from "pinia";
import { ref } from "vue";
import type { User, Permission, UserPermission } from "../../domain/entities";
import { UserApiRepository } from "../../infrastructure/api";
import { LoginUseCase, CreateUserUseCase, GetAllUsersUseCase, UpdateUserUseCase, DeleteUserUseCase, ManagePermissionsUseCase } from "../../application/usecases";

const repository = new UserApiRepository();

export const useAuthStore = defineStore("auth", () => {
  const user = ref<User | null>(null);
  const permissions = ref<string[]>([]);
  const isAuthenticated = ref(false);
  const error = ref<string | null>(null);

  const loginUseCase = new LoginUseCase(repository);

  function hasPermission(permission: string): boolean {
    return permissions.value.includes(permission);
  }

  async function login(username: string, password: string): Promise<boolean> {
    error.value = null;
    try {
      const response = await loginUseCase.execute(username, password);
      user.value = response.user;
      permissions.value = response.permissions;
      isAuthenticated.value = true;
      localStorage.setItem("currentUser", JSON.stringify(response.user));
      localStorage.setItem("userPermissions", JSON.stringify(response.permissions));
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    }
  }

  function logout() {
    user.value = null;
    permissions.value = [];
    isAuthenticated.value = false;
    localStorage.removeItem("currentUser");
    localStorage.removeItem("userPermissions");
  }

  function loadFromStorage() {
    const storedUser = localStorage.getItem("currentUser");
    const storedPermissions = localStorage.getItem("userPermissions");
    if (storedUser && storedPermissions) {
      user.value = JSON.parse(storedUser);
      permissions.value = JSON.parse(storedPermissions);
      isAuthenticated.value = true;
    }
  }

  return {
    user,
    permissions,
    isAuthenticated,
    error,
    login,
    logout,
    hasPermission,
    loadFromStorage,
  };
});

export const useUsersStore = defineStore("users", () => {
  const users = ref<User[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  const createUserUseCase = new CreateUserUseCase(repository);
  const getAllUsersUseCase = new GetAllUsersUseCase(repository);
  const updateUserUseCase = new UpdateUserUseCase(repository);
  const deleteUserUseCase = new DeleteUserUseCase(repository);

  async function fetchUsers() {
    loading.value = true;
    error.value = null;
    try {
      users.value = await getAllUsersUseCase.execute();
    } catch (e) {
      error.value = e as string;
    } finally {
      loading.value = false;
    }
  }

  async function createUser(username: string, password: string): Promise<boolean> {
    error.value = null;
    try {
      const newUser = await createUserUseCase.execute(username, password);
      users.value.push(newUser);
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    }
  }

  async function updateUser(id: number, username: string, active: boolean): Promise<boolean> {
    error.value = null;
    try {
      const updated = await updateUserUseCase.execute(id, username, active);
      const index = users.value.findIndex((u) => u.id === id);
      if (index !== -1) {
        users.value[index] = updated;
      }
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    }
  }

  async function deleteUser(id: number): Promise<boolean> {
    error.value = null;
    try {
      await deleteUserUseCase.execute(id);
      users.value = users.value.filter((u) => u.id !== id);
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    }
  }

  return {
    users,
    loading,
    error,
    fetchUsers,
    createUser,
    updateUser,
    deleteUser,
  };
});

export const usePermissionsStore = defineStore("permissions", () => {
  const allPermissions = ref<Permission[]>([]);
  const userPermissions = ref<Map<number, UserPermission[]>>(new Map());
  const loading = ref(false);
  const error = ref<string | null>(null);

  const managePermissionsUseCase = new ManagePermissionsUseCase(repository);

  async function fetchAllPermissions() {
    loading.value = true;
    error.value = null;
    try {
      allPermissions.value = await managePermissionsUseCase.getAllPermissions();
    } catch (e) {
      error.value = e as string;
    } finally {
      loading.value = false;
    }
  }

  async function fetchUserPermissions(userId: number) {
    error.value = null;
    try {
      const perms = await managePermissionsUseCase.getUserPermissions(userId);
      userPermissions.value.set(userId, perms);
    } catch (e) {
      error.value = e as string;
    }
  }

  async function addPermission(userId: number, permissionId: number): Promise<boolean> {
    error.value = null;
    try {
      await managePermissionsUseCase.addPermission(userId, permissionId);
      await fetchUserPermissions(userId);
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    }
  }

  async function removePermission(userId: number, permissionId: number): Promise<boolean> {
    error.value = null;
    try {
      await managePermissionsUseCase.removePermission(userId, permissionId);
      await fetchUserPermissions(userId);
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    }
  }

  async function createPermission(name: string): Promise<boolean> {
    error.value = null;
    try {
      const newPerm = await managePermissionsUseCase.createPermission(name);
      allPermissions.value.push(newPerm);
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    }
  }

  function getUserPermissions(userId: number): UserPermission[] {
    return userPermissions.value.get(userId) || [];
  }

  return {
    allPermissions,
    userPermissions,
    loading,
    error,
    fetchAllPermissions,
    fetchUserPermissions,
    addPermission,
    removePermission,
    createPermission,
    getUserPermissions,
  };
});

import { ProveedorApiRepository } from "../../infrastructure/api/proveedorRepository";
import type { Proveedor, CreateProveedorRequest, UpdateProveedorRequest } from "../../domain/entities";

const proveedorRepository = new ProveedorApiRepository();

export const useProveedoresStore = defineStore("proveedores", () => {
  const proveedores = ref<Proveedor[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function fetchProveedores() {
    loading.value = true;
    error.value = null;
    try {
      proveedores.value = await proveedorRepository.getAllProveedores();
    } catch (e) {
      error.value = e as string;
    } finally {
      loading.value = false;
    }
  }

  async function createProveedor(request: CreateProveedorRequest): Promise<boolean> {
    error.value = null;
    try {
      const newProveedor = await proveedorRepository.createProveedor(request);
      proveedores.value.push(newProveedor);
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    }
  }

  async function updateProveedor(request: UpdateProveedorRequest): Promise<boolean> {
    error.value = null;
    try {
      const updated = await proveedorRepository.updateProveedor(request);
      const index = proveedores.value.findIndex((p) => p.id === request.id);
      if (index !== -1) {
        proveedores.value[index] = updated;
      }
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    }
  }

  async function deleteProveedor(id: number): Promise<boolean> {
    error.value = null;
    try {
      await proveedorRepository.deleteProveedor(id);
      proveedores.value = proveedores.value.filter((p) => p.id !== id);
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    }
  }

  return {
    proveedores,
    loading,
    error,
    fetchProveedores,
    createProveedor,
    updateProveedor,
    deleteProveedor,
  };
});

import { CategoriaApiRepository } from "../../infrastructure/api/CategoriaRepository";
import type { Categoria, CreateCategoriaRequest, UpdateCategoriaRequest } from "../../domain/entities";

const categoriaRepository = new CategoriaApiRepository();

export const useCategoriasStore = defineStore("categorias", () => {
  const categorias = ref<Categoria[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function fetchCategorias() {
    loading.value = true;
    error.value = null;
    try {
      categorias.value = await categoriaRepository.getAllCategorias();
    } catch (e) {
      error.value = e as string;
    } finally {
      loading.value = false;
    }
  }

  async function createCategoria(request: CreateCategoriaRequest): Promise<boolean> {
    error.value = null;
    try {
      const newCategoria = await categoriaRepository.createCategoria(request);
      categorias.value.push(newCategoria);
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    }
  }

  async function updateCategoria(request: UpdateCategoriaRequest): Promise<boolean> {
    error.value = null;
    try {
      const updated = await categoriaRepository.updateCategoria(request);
      const index = categorias.value.findIndex((c) => c.id === request.id);
      if (index !== -1) {
        categorias.value[index] = updated;
      }
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    }
  }

  async function deleteCategoria(id: number): Promise<boolean> {
    error.value = null;
    try {
      await categoriaRepository.deleteCategoria(id);
      categorias.value = categorias.value.filter((c) => c.id !== id);
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    }
  }

  return {
    categorias,
    loading,
    error,
    fetchCategorias,
    createCategoria,
    updateCategoria,
    deleteCategoria,
  };
});

import { SubCategoriaApiRepository } from "../../infrastructure/api/subCategoriaRepository";
import type { SubCategoria, CreateSubCategoriaRequest, UpdateSubCategoriaRequest } from "../../domain/entities";

const subCategoriaRepository = new SubCategoriaApiRepository();

export const useSubCategoriasStore = defineStore("subCategorias", () => {
  const subCategorias = ref<SubCategoria[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function fetchSubCategorias() {
    loading.value = true;
    error.value = null;
    try {
      subCategorias.value = await subCategoriaRepository.getAllSubCategorias();
    } catch (e) {
      error.value = e as string;
    } finally {
      loading.value = false;
    }
  }

  async function createSubCategoria(request: CreateSubCategoriaRequest): Promise<boolean> {
    error.value = null;
    try {
      const newSubCategoria = await subCategoriaRepository.createSubCategoria(request);
      subCategorias.value.push(newSubCategoria);
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    }
  }

  async function updateSubCategoria(request: UpdateSubCategoriaRequest): Promise<boolean> {
    error.value = null;
    try {
      const updated = await subCategoriaRepository.updateSubCategoria(request);
      const index = subCategorias.value.findIndex((s) => s.id === request.id);
      if (index !== -1) {
        subCategorias.value[index] = updated;
      }
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    }
  }

  async function deleteSubCategoria(id: number): Promise<boolean> {
    error.value = null;
    try {
      await subCategoriaRepository.deleteSubCategoria(id);
      subCategorias.value = subCategorias.value.filter((s) => s.id !== id);
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    }
  }

  return {
    subCategorias,
    loading,
    error,
    fetchSubCategorias,
    createSubCategoria,
    updateSubCategoria,
    deleteSubCategoria,
  };
});

import { ArticuloApiRepository } from "../../infrastructure/api/articuloRepository";
import type { Articulo, CreateArticuloRequest, UpdateArticuloRequest } from "../../domain/entities";

const articuloRepository = new ArticuloApiRepository();

export const useArticulosStore = defineStore("articulos", () => {
  const articulos = ref<Articulo[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function fetchArticulos() {
    loading.value = true;
    error.value = null;
    try {
      articulos.value = await articuloRepository.getAllArticulos();
    } catch (e) {
      error.value = e as string;
    } finally {
      loading.value = false;
    }
  }

  async function createArticulo(request: CreateArticuloRequest): Promise<boolean> {
    error.value = null;
    try {
      const newArticulo = await articuloRepository.createArticulo(request);
      articulos.value.push(newArticulo);
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    }
  }

  async function updateArticulo(request: UpdateArticuloRequest): Promise<boolean> {
    error.value = null;
    try {
      const updated = await articuloRepository.updateArticulo(request);
      const index = articulos.value.findIndex((a) => a.id === request.id);
      if (index !== -1) {
        articulos.value[index] = updated;
      }
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    }
  }

  async function deleteArticulo(id: number): Promise<boolean> {
    error.value = null;
    try {
      await articuloRepository.deleteArticulo(id);
      articulos.value = articulos.value.filter((a) => a.id !== id);
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    }
  }

  return {
    articulos,
    loading,
    error,
    fetchArticulos,
    createArticulo,
    updateArticulo,
    deleteArticulo,
  };
});

import { StockApiRepository } from "../../infrastructure/api/stockRepository";
import type { Stock, CreateStockRequest, UpdateStockRequest } from "../../domain/entities";

const stockRepository = new StockApiRepository();

export const useStockStore = defineStore("stock", () => {
  const stocks = ref<Stock[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function fetchStock() {
    loading.value = true;
    error.value = null;
    try {
      stocks.value = await stockRepository.getAllStock();
    } catch (e) {
      error.value = e as string;
    } finally {
      loading.value = false;
    }
  }

  async function getStockByArticulo(idArticulo: number): Promise<Stock | null> {
    try {
      return await stockRepository.getStockByArticulo(idArticulo);
    } catch (e) {
      error.value = e as string;
      return null;
    }
  }

  async function createStock(request: CreateStockRequest): Promise<boolean> {
    error.value = null;
    try {
      const newStock = await stockRepository.createStock(request);
      stocks.value.push(newStock);
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    }
  }

  async function updateStock(request: UpdateStockRequest): Promise<boolean> {
    error.value = null;
    try {
      const updated = await stockRepository.updateStock(request);
      const index = stocks.value.findIndex((s) => s.id === request.id);
      if (index !== -1) {
        stocks.value[index] = updated;
      }
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    }
  }

  async function deleteStock(id: number): Promise<boolean> {
    error.value = null;
    try {
      await stockRepository.deleteStock(id);
      stocks.value = stocks.value.filter((s) => s.id !== id);
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    }
  }

  async function getPrecioVenta(id: number): Promise<number | null> {
    try {
      return await stockRepository.getPrecioVenta(id);
    } catch (e) {
      error.value = e as string;
      return null;
    }
  }

  function calcularPrecioVenta(costo: number, ganancia: number): number {
    return costo * (1 + ganancia / 100);
  }

  return {
    stocks,
    loading,
    error,
    fetchStock,
    getStockByArticulo,
    createStock,
    updateStock,
    deleteStock,
    getPrecioVenta,
    calcularPrecioVenta,
  };
});
